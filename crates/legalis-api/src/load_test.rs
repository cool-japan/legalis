//! Load testing utilities for API performance testing.
//!
//! This module provides utilities for load testing API endpoints,
//! including concurrent request simulation, performance metrics collection,
//! and scenario-based testing.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Load test configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestConfig {
    /// Number of concurrent users/connections
    pub concurrent_users: usize,
    /// Total number of requests to send
    pub total_requests: usize,
    /// Duration to run the test (overrides total_requests if set)
    pub duration: Option<Duration>,
    /// Ramp-up time to gradually increase load
    pub ramp_up_duration: Option<Duration>,
    /// Delay between requests per user
    pub request_delay: Option<Duration>,
    /// Timeout for each request
    pub request_timeout: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_users: 10,
            total_requests: 100,
            duration: None,
            ramp_up_duration: None,
            request_delay: None,
            request_timeout: Duration::from_secs(30),
        }
    }
}

/// Request result from load testing.
#[derive(Debug, Clone)]
pub struct RequestResult {
    /// Request duration
    pub duration: Duration,
    /// HTTP status code
    pub status_code: u16,
    /// Whether the request succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Response size in bytes
    pub response_size: usize,
}

/// Load test results and statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestResults {
    /// Total requests sent
    pub total_requests: usize,
    /// Successful requests
    pub successful_requests: usize,
    /// Failed requests
    pub failed_requests: usize,
    /// Total test duration
    pub total_duration: Duration,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time
    pub avg_response_time_ms: f64,
    /// Minimum response time
    pub min_response_time_ms: f64,
    /// Maximum response time
    pub max_response_time_ms: f64,
    /// 50th percentile (median)
    pub p50_response_time_ms: f64,
    /// 95th percentile
    pub p95_response_time_ms: f64,
    /// 99th percentile
    pub p99_response_time_ms: f64,
    /// Total bytes transferred
    pub total_bytes: usize,
    /// Error rate (0.0 - 1.0)
    pub error_rate: f64,
}

/// Load test scenario executor.
pub struct LoadTester {
    config: LoadTestConfig,
}

impl LoadTester {
    /// Creates a new load tester with the given configuration.
    pub fn new(config: LoadTestConfig) -> Self {
        Self { config }
    }

    /// Executes a load test with the provided request function.
    ///
    /// The request function should perform a single API request and return the result.
    pub async fn run<F, Fut>(&self, request_fn: F) -> LoadTestResults
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = RequestResult> + Send,
    {
        let start_time = Instant::now();
        let request_fn = Arc::new(request_fn);
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));

        let total_requests = if let Some(duration) = self.config.duration {
            // Estimate requests based on duration and delay
            let requests_per_user = if let Some(delay) = self.config.request_delay {
                (duration.as_secs_f64() / delay.as_secs_f64()) as usize
            } else {
                1000 // Default high number
            };
            self.config.concurrent_users * requests_per_user
        } else {
            self.config.total_requests
        };

        let mut tasks = Vec::new();
        let mut request_count = 0;

        // Ramp-up phase
        let ramp_up_delay = self
            .config
            .ramp_up_duration
            .map(|ramp_up| ramp_up.as_millis() as u64 / self.config.concurrent_users as u64);

        while request_count < total_requests {
            // Check if duration limit reached
            if let Some(duration) = self.config.duration {
                if start_time.elapsed() >= duration {
                    break;
                }
            }

            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let request_fn = request_fn.clone();
            let request_delay = self.config.request_delay;

            let task = tokio::spawn(async move {
                let result = request_fn().await;

                // Apply request delay if configured
                if let Some(delay) = request_delay {
                    tokio::time::sleep(delay).await;
                }

                drop(permit);
                result
            });

            tasks.push(task);
            request_count += 1;

            // Apply ramp-up delay
            if let Some(delay_ms) = ramp_up_delay {
                if request_count < self.config.concurrent_users {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }

        // Wait for all requests to complete
        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }

        let total_duration = start_time.elapsed();
        self.calculate_statistics(results, total_duration)
    }

    /// Calculates statistics from request results.
    fn calculate_statistics(
        &self,
        results: Vec<RequestResult>,
        total_duration: Duration,
    ) -> LoadTestResults {
        let total_requests = results.len();
        let successful_requests = results.iter().filter(|r| r.success).count();
        let failed_requests = total_requests - successful_requests;

        let mut durations: Vec<f64> = results
            .iter()
            .map(|r| r.duration.as_secs_f64() * 1000.0)
            .collect();
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let avg_response_time_ms = if !durations.is_empty() {
            durations.iter().sum::<f64>() / durations.len() as f64
        } else {
            0.0
        };

        let min_response_time_ms = durations.first().copied().unwrap_or(0.0);
        let max_response_time_ms = durations.last().copied().unwrap_or(0.0);

        let p50_response_time_ms = self.percentile(&durations, 0.50);
        let p95_response_time_ms = self.percentile(&durations, 0.95);
        let p99_response_time_ms = self.percentile(&durations, 0.99);

        let total_bytes = results.iter().map(|r| r.response_size).sum();
        let requests_per_second = total_requests as f64 / total_duration.as_secs_f64();
        let error_rate = failed_requests as f64 / total_requests as f64;

        LoadTestResults {
            total_requests,
            successful_requests,
            failed_requests,
            total_duration,
            requests_per_second,
            avg_response_time_ms,
            min_response_time_ms,
            max_response_time_ms,
            p50_response_time_ms,
            p95_response_time_ms,
            p99_response_time_ms,
            total_bytes,
            error_rate,
        }
    }

    /// Calculates percentile from sorted values.
    fn percentile(&self, sorted_values: &[f64], percentile: f64) -> f64 {
        if sorted_values.is_empty() {
            return 0.0;
        }

        let index = (percentile * (sorted_values.len() - 1) as f64) as usize;
        sorted_values[index]
    }
}

/// Pre-defined load test scenarios.
pub mod scenarios {
    use super::*;

    /// Light load: 10 users, 100 requests
    pub fn light_load() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 10,
            total_requests: 100,
            duration: None,
            ramp_up_duration: Some(Duration::from_secs(2)),
            request_delay: Some(Duration::from_millis(100)),
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Medium load: 50 users, 500 requests
    pub fn medium_load() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 50,
            total_requests: 500,
            duration: None,
            ramp_up_duration: Some(Duration::from_secs(5)),
            request_delay: Some(Duration::from_millis(50)),
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Heavy load: 100 users, 1000 requests
    pub fn heavy_load() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 100,
            total_requests: 1000,
            duration: None,
            ramp_up_duration: Some(Duration::from_secs(10)),
            request_delay: Some(Duration::from_millis(10)),
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Stress test: 200 users for 60 seconds
    pub fn stress_test() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 200,
            total_requests: 10000,
            duration: Some(Duration::from_secs(60)),
            ramp_up_duration: Some(Duration::from_secs(10)),
            request_delay: None,
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Spike test: Sudden increase to 500 users
    pub fn spike_test() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 500,
            total_requests: 5000,
            duration: Some(Duration::from_secs(30)),
            ramp_up_duration: None, // No ramp-up for spike
            request_delay: None,
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Endurance test: Moderate load for extended period
    pub fn endurance_test() -> LoadTestConfig {
        LoadTestConfig {
            concurrent_users: 50,
            total_requests: 100000,
            duration: Some(Duration::from_secs(600)), // 10 minutes
            ramp_up_duration: Some(Duration::from_secs(30)),
            request_delay: Some(Duration::from_millis(100)),
            request_timeout: Duration::from_secs(30),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_tester_basic() {
        let config = LoadTestConfig {
            concurrent_users: 5,
            total_requests: 20,
            duration: None,
            ramp_up_duration: None,
            request_delay: None,
            request_timeout: Duration::from_secs(5),
        };

        let tester = LoadTester::new(config);

        let results = tester
            .run(|| async {
                // Simulate a fast successful request
                tokio::time::sleep(Duration::from_millis(10)).await;
                RequestResult {
                    duration: Duration::from_millis(10),
                    status_code: 200,
                    success: true,
                    error: None,
                    response_size: 100,
                }
            })
            .await;

        assert_eq!(results.total_requests, 20);
        assert_eq!(results.successful_requests, 20);
        assert_eq!(results.failed_requests, 0);
        assert!(results.avg_response_time_ms >= 10.0);
        assert!(results.requests_per_second > 0.0);
    }

    #[tokio::test]
    async fn test_load_tester_with_failures() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let config = LoadTestConfig {
            concurrent_users: 2,
            total_requests: 10,
            duration: None,
            ramp_up_duration: None,
            request_delay: None,
            request_timeout: Duration::from_secs(5),
        };

        let tester = LoadTester::new(config);
        let counter = Arc::new(AtomicUsize::new(0));

        let results = tester
            .run(move || {
                let counter = counter.clone();
                async move {
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    let should_fail = count % 3 == 0;
                    RequestResult {
                        duration: Duration::from_millis(5),
                        status_code: if should_fail { 500 } else { 200 },
                        success: !should_fail,
                        error: if should_fail {
                            Some("Simulated error".to_string())
                        } else {
                            None
                        },
                        response_size: 50,
                    }
                }
            })
            .await;

        assert_eq!(results.total_requests, 10);
        assert!(results.failed_requests > 0);
        assert!(results.error_rate > 0.0);
    }

    #[test]
    fn test_load_test_config_default() {
        let config = LoadTestConfig::default();
        assert_eq!(config.concurrent_users, 10);
        assert_eq!(config.total_requests, 100);
        assert_eq!(config.request_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_scenario_light_load() {
        let config = scenarios::light_load();
        assert_eq!(config.concurrent_users, 10);
        assert_eq!(config.total_requests, 100);
    }

    #[test]
    fn test_scenario_heavy_load() {
        let config = scenarios::heavy_load();
        assert_eq!(config.concurrent_users, 100);
        assert_eq!(config.total_requests, 1000);
    }

    #[tokio::test]
    async fn test_percentile_calculation() {
        let config = LoadTestConfig::default();
        let tester = LoadTester::new(config);

        let durations = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let p50 = tester.percentile(&durations, 0.50);
        let p95 = tester.percentile(&durations, 0.95);
        let p99 = tester.percentile(&durations, 0.99);

        assert!((5.0..=6.0).contains(&p50));
        assert!(p95 >= 9.0);
        assert!(p99 >= 9.0);
    }

    #[tokio::test]
    async fn test_ramp_up() {
        let config = LoadTestConfig {
            concurrent_users: 5,
            total_requests: 10,
            duration: None,
            ramp_up_duration: Some(Duration::from_millis(100)),
            request_delay: None,
            request_timeout: Duration::from_secs(5),
        };

        let tester = LoadTester::new(config);
        let start = Instant::now();

        let _results = tester
            .run(|| async {
                RequestResult {
                    duration: Duration::from_millis(1),
                    status_code: 200,
                    success: true,
                    error: None,
                    response_size: 10,
                }
            })
            .await;

        // Ramp-up should add some delay
        assert!(start.elapsed() >= Duration::from_millis(50));
    }
}
