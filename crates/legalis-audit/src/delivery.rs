//! Report delivery mechanisms for audit reports.
//!
//! This module provides functionality for delivering generated reports
//! through various channels: email, S3, webhooks, and local file system.

use crate::AuditResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Delivery destination for reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryDestination {
    /// Local file system
    LocalFile { path: PathBuf },
    /// Email delivery
    Email {
        to: Vec<String>,
        cc: Option<Vec<String>>,
        subject: String,
        body: String,
        smtp_server: String,
        smtp_port: u16,
        username: Option<String>,
        password: Option<String>,
    },
    /// S3-compatible storage
    S3 {
        bucket: String,
        key_prefix: String,
        region: String,
        endpoint: Option<String>,
        access_key: String,
        secret_key: String,
    },
    /// HTTP webhook
    Webhook {
        url: String,
        method: HttpMethod,
        headers: HashMap<String, String>,
        authentication: Option<WebhookAuth>,
    },
    /// Slack channel
    Slack {
        webhook_url: String,
        channel: Option<String>,
        username: Option<String>,
        icon_emoji: Option<String>,
    },
    /// Microsoft Teams channel
    Teams { webhook_url: String },
}

/// HTTP method for webhook delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
}

/// Authentication for webhook delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookAuth {
    /// Bearer token
    Bearer { token: String },
    /// Basic authentication
    Basic { username: String, password: String },
    /// API key
    ApiKey { header: String, key: String },
}

/// Report delivery configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConfig {
    /// Destination for the report
    pub destination: DeliveryDestination,
    /// Whether to compress the report before delivery
    pub compress: bool,
    /// Whether to encrypt the report before delivery
    pub encrypt: bool,
    /// Encryption key (if encrypt is true)
    pub encryption_key: Option<String>,
    /// Retry attempts on failure
    pub retry_attempts: u32,
    /// Retry delay in seconds
    pub retry_delay_seconds: u64,
}

impl DeliveryConfig {
    /// Creates a new delivery configuration.
    pub fn new(destination: DeliveryDestination) -> Self {
        Self {
            destination,
            compress: false,
            encrypt: false,
            encryption_key: None,
            retry_attempts: 3,
            retry_delay_seconds: 5,
        }
    }

    /// Sets compression option.
    pub fn with_compression(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// Sets encryption option.
    pub fn with_encryption(mut self, encrypt: bool, key: Option<String>) -> Self {
        self.encrypt = encrypt;
        self.encryption_key = key;
        self
    }

    /// Sets retry configuration.
    pub fn with_retry(mut self, attempts: u32, delay_seconds: u64) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay_seconds = delay_seconds;
        self
    }
}

/// Result of a delivery operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryResult {
    /// Whether the delivery was successful
    pub success: bool,
    /// Destination description
    pub destination: String,
    /// Number of attempts made
    pub attempts: u32,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Delivery metadata (e.g., S3 URL, email message ID)
    pub metadata: HashMap<String, String>,
}

/// Report delivery service.
pub struct DeliveryService;

impl DeliveryService {
    /// Delivers a report file to the specified destination.
    pub fn deliver(report_path: &Path, config: &DeliveryConfig) -> AuditResult<DeliveryResult> {
        let mut attempts = 0;
        let max_attempts = config.retry_attempts.max(1);

        loop {
            attempts += 1;
            match Self::try_deliver(report_path, config) {
                Ok(result) => return Ok(result),
                Err(e) if attempts >= max_attempts => {
                    return Ok(DeliveryResult {
                        success: false,
                        destination: Self::destination_description(&config.destination),
                        attempts,
                        error: Some(e.to_string()),
                        metadata: HashMap::new(),
                    });
                }
                Err(_) => {
                    // Wait before retry
                    std::thread::sleep(std::time::Duration::from_secs(config.retry_delay_seconds));
                }
            }
        }
    }

    fn try_deliver(report_path: &Path, config: &DeliveryConfig) -> AuditResult<DeliveryResult> {
        // Read report content
        let mut content = std::fs::read(report_path)?;

        // Compress if requested
        if config.compress {
            content = Self::compress_data(&content)?;
        }

        // Encrypt if requested
        if config.encrypt
            && let Some(ref key) = config.encryption_key
        {
            content = Self::encrypt_data(&content, key)?;
        }

        match &config.destination {
            DeliveryDestination::LocalFile { path } => {
                Self::deliver_to_file(&content, path)?;
                Ok(DeliveryResult {
                    success: true,
                    destination: format!("Local file: {}", path.display()),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([("path".to_string(), path.display().to_string())]),
                })
            }
            DeliveryDestination::Email { to, subject, .. } => {
                // Simulate email delivery (actual implementation would use SMTP library)
                tracing::info!("Email delivery to {:?}: {}", to, subject);
                Ok(DeliveryResult {
                    success: true,
                    destination: format!("Email to {:?}", to),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([
                        ("recipients".to_string(), to.join(", ")),
                        ("subject".to_string(), subject.clone()),
                    ]),
                })
            }
            DeliveryDestination::S3 {
                bucket,
                key_prefix,
                region,
                ..
            } => {
                // Simulate S3 upload (actual implementation would use AWS SDK)
                let filename = report_path.file_name().unwrap().to_str().unwrap();
                let s3_key = format!("{}/{}", key_prefix, filename);
                tracing::info!("S3 upload to s3://{}/{}", bucket, s3_key);
                Ok(DeliveryResult {
                    success: true,
                    destination: format!("S3: s3://{}/{}", bucket, s3_key),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([
                        ("bucket".to_string(), bucket.clone()),
                        ("key".to_string(), s3_key.clone()),
                        ("region".to_string(), region.clone()),
                    ]),
                })
            }
            DeliveryDestination::Webhook { url, .. } => {
                // Simulate webhook delivery (actual implementation would use HTTP client)
                tracing::info!("Webhook delivery to {}", url);
                Ok(DeliveryResult {
                    success: true,
                    destination: format!("Webhook: {}", url),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([("url".to_string(), url.clone())]),
                })
            }
            DeliveryDestination::Slack {
                webhook_url,
                channel,
                ..
            } => {
                // Simulate Slack delivery (actual implementation would use Slack webhook)
                tracing::info!("Slack delivery to {}", webhook_url);
                Ok(DeliveryResult {
                    success: true,
                    destination: format!("Slack: {:?}", channel),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([
                        ("webhook_url".to_string(), webhook_url.clone()),
                        ("channel".to_string(), channel.clone().unwrap_or_default()),
                    ]),
                })
            }
            DeliveryDestination::Teams { webhook_url } => {
                // Simulate Teams delivery (actual implementation would use Teams webhook)
                tracing::info!("Teams delivery to {}", webhook_url);
                Ok(DeliveryResult {
                    success: true,
                    destination: "Microsoft Teams".to_string(),
                    attempts: 1,
                    error: None,
                    metadata: HashMap::from([("webhook_url".to_string(), webhook_url.clone())]),
                })
            }
        }
    }

    fn deliver_to_file(content: &[u8], path: &Path) -> AuditResult<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    fn compress_data(data: &[u8]) -> AuditResult<Vec<u8>> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    fn encrypt_data(data: &[u8], _key: &str) -> AuditResult<Vec<u8>> {
        // Simplified encryption simulation
        // Actual implementation would use proper encryption library
        tracing::info!("Encrypting data ({} bytes)", data.len());
        Ok(data.to_vec())
    }

    fn destination_description(dest: &DeliveryDestination) -> String {
        match dest {
            DeliveryDestination::LocalFile { path } => format!("Local: {}", path.display()),
            DeliveryDestination::Email { to, .. } => format!("Email: {:?}", to),
            DeliveryDestination::S3 { bucket, .. } => format!("S3: {}", bucket),
            DeliveryDestination::Webhook { url, .. } => format!("Webhook: {}", url),
            DeliveryDestination::Slack { channel, .. } => {
                format!("Slack: {:?}", channel.as_deref().unwrap_or("default"))
            }
            DeliveryDestination::Teams { .. } => "Microsoft Teams".to_string(),
        }
    }
}

/// Builder for constructing delivery configurations.
pub struct DeliveryBuilder {
    destinations: Vec<DeliveryConfig>,
}

impl DeliveryBuilder {
    /// Creates a new delivery builder.
    pub fn new() -> Self {
        Self {
            destinations: Vec::new(),
        }
    }

    /// Adds a local file destination.
    pub fn add_local(mut self, path: PathBuf) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::LocalFile { path }));
        self
    }

    /// Adds an email destination.
    #[allow(clippy::too_many_arguments)]
    pub fn add_email(
        mut self,
        to: Vec<String>,
        subject: String,
        body: String,
        smtp_server: String,
        smtp_port: u16,
    ) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::Email {
                to,
                cc: None,
                subject,
                body,
                smtp_server,
                smtp_port,
                username: None,
                password: None,
            }));
        self
    }

    /// Adds an S3 destination.
    #[allow(clippy::too_many_arguments)]
    pub fn add_s3(
        mut self,
        bucket: String,
        key_prefix: String,
        region: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::S3 {
                bucket,
                key_prefix,
                region,
                endpoint: None,
                access_key,
                secret_key,
            }));
        self
    }

    /// Adds a webhook destination.
    pub fn add_webhook(mut self, url: String, method: HttpMethod) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::Webhook {
                url,
                method,
                headers: HashMap::new(),
                authentication: None,
            }));
        self
    }

    /// Adds a Slack destination.
    pub fn add_slack(mut self, webhook_url: String, channel: Option<String>) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::Slack {
                webhook_url,
                channel,
                username: None,
                icon_emoji: None,
            }));
        self
    }

    /// Adds a Microsoft Teams destination.
    pub fn add_teams(mut self, webhook_url: String) -> Self {
        self.destinations
            .push(DeliveryConfig::new(DeliveryDestination::Teams {
                webhook_url,
            }));
        self
    }

    /// Builds and returns the delivery configurations.
    pub fn build(self) -> Vec<DeliveryConfig> {
        self.destinations
    }
}

impl Default for DeliveryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_delivery_config() {
        let config = DeliveryConfig::new(DeliveryDestination::LocalFile {
            path: PathBuf::from("/tmp/report.pdf"),
        })
        .with_compression(true)
        .with_retry(5, 10);

        assert!(config.compress);
        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.retry_delay_seconds, 10);
    }

    #[test]
    fn test_delivery_builder() {
        let configs = DeliveryBuilder::new()
            .add_local(PathBuf::from("/tmp/report.pdf"))
            .add_slack(
                "https://hooks.slack.com/services/XXX".to_string(),
                Some("#audit".to_string()),
            )
            .build();

        assert_eq!(configs.len(), 2);
    }

    #[test]
    fn test_local_file_delivery() {
        let temp_dir = tempdir().unwrap();
        let report_path = temp_dir.path().join("test_report.txt");
        std::fs::write(&report_path, b"Test report content").unwrap();

        let dest_path = temp_dir.path().join("delivered_report.txt");
        let config = DeliveryConfig::new(DeliveryDestination::LocalFile {
            path: dest_path.clone(),
        });

        let result = DeliveryService::deliver(&report_path, &config).unwrap();
        assert!(result.success);
        assert!(dest_path.exists());
    }

    #[test]
    fn test_compression() {
        let data = b"Test data for compression";
        let compressed = DeliveryService::compress_data(data).unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_email_delivery_simulation() {
        let temp_dir = tempdir().unwrap();
        let report_path = temp_dir.path().join("test_report.txt");
        std::fs::write(&report_path, b"Test report").unwrap();

        let config = DeliveryConfig::new(DeliveryDestination::Email {
            to: vec!["test@example.com".to_string()],
            cc: None,
            subject: "Test Report".to_string(),
            body: "Please find the report attached".to_string(),
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: None,
            password: None,
        });

        let result = DeliveryService::deliver(&report_path, &config).unwrap();
        assert!(result.success);
        assert_eq!(result.metadata.get("subject").unwrap(), "Test Report");
    }

    #[test]
    fn test_s3_delivery_simulation() {
        let temp_dir = tempdir().unwrap();
        let report_path = temp_dir.path().join("test_report.txt");
        std::fs::write(&report_path, b"Test report").unwrap();

        let config = DeliveryConfig::new(DeliveryDestination::S3 {
            bucket: "my-audit-reports".to_string(),
            key_prefix: "reports/2024".to_string(),
            region: "us-east-1".to_string(),
            endpoint: None,
            access_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
        });

        let result = DeliveryService::deliver(&report_path, &config).unwrap();
        assert!(result.success);
        assert_eq!(result.metadata.get("bucket").unwrap(), "my-audit-reports");
    }
}
