//! High-resolution, allocation-aware execution profiler for the Legalis CLI.
//!
//! This module is deliberately distinct from [`crate::perf`]. Where `perf`
//! samples coarse OS-level RSS and persists per-command sessions to disk, this
//! module focuses on *in-process micro-profiling*:
//!
//! - A pure-Rust instrumented [`TrackingAllocator`] that wraps any
//!   [`GlobalAlloc`] and accounts for live / peak / total heap bytes using
//!   lock-free atomics. It can be installed as the process `#[global_allocator]`
//!   so the running CLI tracks its own heap without any external service.
//! - Phase-scoped, high-resolution timing via [`Instant`] ([`Profiler::measure`]).
//! - Statistical aggregation over repeated samples: mean, population standard
//!   deviation, and linear-interpolated percentiles (p50/p90/p95/p99).
//! - Bottleneck detection across phases (time share, latency, variance,
//!   allocation pressure) and heuristic [`OptimizationHint`]s.
//! - Structured [`ProfileReport`]s rendered through the crate's [`OutputFormat`].
//!
//! The profiler reads memory statistics through the [`MemorySource`] trait, which
//! decouples the timing logic from how bytes are measured. Three sources are
//! provided: [`AllocatorSource`] (the instrumented allocator), [`RssMemorySource`]
//! (OS resident-set sampling), and [`ManualMemorySource`] (deterministic, used in
//! tests and for injecting synthetic workloads).

use crate::OutputFormat;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Converts a [`Duration`] to fractional milliseconds.
fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

// ---------------------------------------------------------------------------
// Instrumented allocator
// ---------------------------------------------------------------------------

/// A snapshot of an allocator's accounting counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocStats {
    /// Cumulative bytes ever requested from the allocator.
    pub allocated: u64,
    /// Cumulative bytes ever returned to the allocator.
    pub deallocated: u64,
    /// Currently live bytes (`allocated - deallocated`).
    pub live: u64,
    /// Maximum live bytes observed since the last peak reset.
    pub peak: u64,
    /// Number of allocation events.
    pub alloc_count: u64,
    /// Number of deallocation events.
    pub dealloc_count: u64,
}

/// An allocator wrapper that accounts for heap usage using atomic counters.
///
/// Wrapping the system allocator and installing it as the global allocator lets
/// the CLI track its own memory footprint without any external profiler. All
/// counters use [`Ordering::Relaxed`]; the peak is maintained with a small
/// compare-and-swap loop so concurrent threads never lose a maximum.
pub struct TrackingAllocator<A> {
    inner: A,
    allocated: AtomicUsize,
    deallocated: AtomicUsize,
    live: AtomicUsize,
    peak: AtomicUsize,
    alloc_count: AtomicU64,
    dealloc_count: AtomicU64,
}

impl<A> TrackingAllocator<A> {
    /// Creates a new tracking allocator wrapping `inner`.
    ///
    /// This is a `const fn` so the allocator can be placed in a `static` and
    /// referenced from `#[global_allocator]`.
    pub const fn new(inner: A) -> Self {
        Self {
            inner,
            allocated: AtomicUsize::new(0),
            deallocated: AtomicUsize::new(0),
            live: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
            alloc_count: AtomicU64::new(0),
            dealloc_count: AtomicU64::new(0),
        }
    }

    /// Currently live bytes.
    pub fn live(&self) -> u64 {
        self.live.load(Ordering::Relaxed) as u64
    }

    /// Maximum live bytes observed since the last [`reset_peak`](Self::reset_peak).
    pub fn peak(&self) -> u64 {
        self.peak.load(Ordering::Relaxed) as u64
    }

    /// Cumulative bytes ever allocated.
    pub fn total_allocated(&self) -> u64 {
        self.allocated.load(Ordering::Relaxed) as u64
    }

    /// Cumulative bytes ever deallocated.
    pub fn total_deallocated(&self) -> u64 {
        self.deallocated.load(Ordering::Relaxed) as u64
    }

    /// Number of allocation events recorded.
    pub fn allocation_count(&self) -> u64 {
        self.alloc_count.load(Ordering::Relaxed)
    }

    /// Number of deallocation events recorded.
    pub fn deallocation_count(&self) -> u64 {
        self.dealloc_count.load(Ordering::Relaxed)
    }

    /// Resets the tracked peak to the current live value.
    ///
    /// Used to scope peak measurement to a single profiling phase.
    pub fn reset_peak(&self) {
        let live = self.live.load(Ordering::Relaxed);
        self.peak.store(live, Ordering::Relaxed);
    }

    /// Returns a consistent snapshot of all counters.
    pub fn snapshot(&self) -> AllocStats {
        AllocStats {
            allocated: self.total_allocated(),
            deallocated: self.total_deallocated(),
            live: self.live(),
            peak: self.peak(),
            alloc_count: self.allocation_count(),
            dealloc_count: self.deallocation_count(),
        }
    }

    /// Raises the recorded peak to `live` if it exceeds the current peak.
    fn raise_peak(&self, live: usize) {
        let mut peak = self.peak.load(Ordering::Relaxed);
        while live > peak {
            match self
                .peak
                .compare_exchange_weak(peak, live, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(observed) => peak = observed,
            }
        }
    }

    /// Accounts for a successful allocation of `size` bytes.
    fn on_alloc(&self, size: usize) {
        self.allocated.fetch_add(size, Ordering::Relaxed);
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        let live = self.live.fetch_add(size, Ordering::Relaxed) + size;
        self.raise_peak(live);
    }

    /// Accounts for a deallocation of `size` bytes.
    fn on_dealloc(&self, size: usize) {
        self.deallocated.fetch_add(size, Ordering::Relaxed);
        self.dealloc_count.fetch_add(1, Ordering::Relaxed);
        self.live.fetch_sub(size, Ordering::Relaxed);
    }

    /// Accounts for an in-place reallocation from `old` to `new` bytes.
    fn on_realloc(&self, old: usize, new: usize) {
        if new >= old {
            let diff = new - old;
            self.allocated.fetch_add(diff, Ordering::Relaxed);
            self.alloc_count.fetch_add(1, Ordering::Relaxed);
            let live = self.live.fetch_add(diff, Ordering::Relaxed) + diff;
            self.raise_peak(live);
        } else {
            let diff = old - new;
            self.deallocated.fetch_add(diff, Ordering::Relaxed);
            self.dealloc_count.fetch_add(1, Ordering::Relaxed);
            self.live.fetch_sub(diff, Ordering::Relaxed);
        }
    }
}

// SAFETY: every method delegates to the wrapped allocator and only touches
// atomic counters in addition. No allocation is performed inside these methods.
unsafe impl<A: GlobalAlloc> GlobalAlloc for TrackingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { self.inner.alloc(layout) };
        if !ptr.is_null() {
            self.on_alloc(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.inner.dealloc(ptr, layout) };
        self.on_dealloc(layout.size());
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { self.inner.alloc_zeroed(layout) };
        if !ptr.is_null() {
            self.on_alloc(layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { self.inner.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            self.on_realloc(layout.size(), new_size);
        }
        new_ptr
    }
}

// SAFETY: a shared reference simply forwards to the underlying allocator, which
// is the pattern required to install a `static` instance as the global
// allocator (see `main.rs`).
unsafe impl<A: GlobalAlloc> GlobalAlloc for &TrackingAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { (**self).alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { (**self).dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { (**self).alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { (**self).realloc(ptr, layout, new_size) }
    }
}

/// Process-wide instrumented allocator instance.
///
/// Install it in the binary with:
///
/// ```ignore
/// #[global_allocator]
/// static GLOBAL_ALLOC: &legalis::profiling::TrackingAllocator<std::alloc::System> =
///     &legalis::profiling::GLOBAL_TRACKING_ALLOCATOR;
/// ```
pub static GLOBAL_TRACKING_ALLOCATOR: TrackingAllocator<System> = TrackingAllocator::new(System);

// ---------------------------------------------------------------------------
// Memory sources
// ---------------------------------------------------------------------------

/// An abstract source of heap-usage statistics.
///
/// Decouples the [`Profiler`] from how memory is measured so that the same
/// timing logic works against the instrumented allocator, OS RSS sampling, or a
/// deterministic in-memory model for tests.
pub trait MemorySource: Send + Sync {
    /// Currently live bytes.
    fn live_bytes(&self) -> u64;
    /// Peak live bytes since the last baseline reset.
    fn peak_bytes(&self) -> u64;
    /// Cumulative bytes ever allocated.
    fn total_allocated_bytes(&self) -> u64;
    /// Number of allocation events.
    fn allocation_events(&self) -> u64;
    /// Resets the peak baseline to the current live value.
    fn mark_peak_baseline(&self);
}

impl<A: GlobalAlloc + Send + Sync> MemorySource for TrackingAllocator<A> {
    fn live_bytes(&self) -> u64 {
        self.live()
    }
    fn peak_bytes(&self) -> u64 {
        self.peak()
    }
    fn total_allocated_bytes(&self) -> u64 {
        self.total_allocated()
    }
    fn allocation_events(&self) -> u64 {
        self.allocation_count()
    }
    fn mark_peak_baseline(&self) {
        self.reset_peak();
    }
}

/// A [`MemorySource`] backed by a `'static` instrumented allocator.
pub struct AllocatorSource {
    allocator: &'static TrackingAllocator<System>,
}

impl AllocatorSource {
    /// Reads from the process-wide [`GLOBAL_TRACKING_ALLOCATOR`].
    pub fn global() -> Self {
        Self {
            allocator: &GLOBAL_TRACKING_ALLOCATOR,
        }
    }

    /// Reads from a specific `'static` tracking allocator.
    pub fn new(allocator: &'static TrackingAllocator<System>) -> Self {
        Self { allocator }
    }
}

impl MemorySource for AllocatorSource {
    fn live_bytes(&self) -> u64 {
        self.allocator.live()
    }
    fn peak_bytes(&self) -> u64 {
        self.allocator.peak()
    }
    fn total_allocated_bytes(&self) -> u64 {
        self.allocator.total_allocated()
    }
    fn allocation_events(&self) -> u64 {
        self.allocator.allocation_count()
    }
    fn mark_peak_baseline(&self) {
        self.allocator.reset_peak();
    }
}

/// A [`MemorySource`] that samples the OS resident-set size.
///
/// On Linux it parses `VmRSS` from `/proc/self/status`; on other platforms it
/// reports zero. The peak is tracked internally so callers still observe a
/// per-phase maximum.
pub struct RssMemorySource {
    peak: AtomicU64,
}

impl RssMemorySource {
    /// Creates a new RSS sampler with the peak primed to the current RSS.
    pub fn new() -> Self {
        let source = Self {
            peak: AtomicU64::new(0),
        };
        let current = source.current_rss();
        source.peak.store(current, Ordering::Relaxed);
        source
    }

    /// Returns the current resident-set size in bytes (0 if unavailable).
    #[cfg(target_os = "linux")]
    fn current_rss(&self) -> u64 {
        let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
            return 0;
        };
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                let kb: u64 = rest
                    .split_whitespace()
                    .next()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0);
                return kb * 1024;
            }
        }
        0
    }

    /// Non-Linux fallback: RSS sampling is unavailable.
    #[cfg(not(target_os = "linux"))]
    fn current_rss(&self) -> u64 {
        0
    }
}

impl Default for RssMemorySource {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorySource for RssMemorySource {
    fn live_bytes(&self) -> u64 {
        let current = self.current_rss();
        let mut peak = self.peak.load(Ordering::Relaxed);
        while current > peak {
            match self.peak.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => peak = observed,
            }
        }
        current
    }
    fn peak_bytes(&self) -> u64 {
        self.peak.load(Ordering::Relaxed).max(self.current_rss())
    }
    fn total_allocated_bytes(&self) -> u64 {
        self.current_rss()
    }
    fn allocation_events(&self) -> u64 {
        0
    }
    fn mark_peak_baseline(&self) {
        let current = self.current_rss();
        self.peak.store(current, Ordering::Relaxed);
    }
}

/// A deterministic in-memory [`MemorySource`] for tests and synthetic workloads.
pub struct ManualMemorySource {
    live: AtomicU64,
    peak: AtomicU64,
    total: AtomicU64,
    events: AtomicU64,
}

impl ManualMemorySource {
    /// Creates a zeroed manual source.
    pub fn new() -> Self {
        Self {
            live: AtomicU64::new(0),
            peak: AtomicU64::new(0),
            total: AtomicU64::new(0),
            events: AtomicU64::new(0),
        }
    }

    /// Simulates an allocation of `bytes`.
    pub fn allocate(&self, bytes: u64) {
        self.total.fetch_add(bytes, Ordering::Relaxed);
        self.events.fetch_add(1, Ordering::Relaxed);
        let live = self.live.fetch_add(bytes, Ordering::Relaxed) + bytes;
        let mut peak = self.peak.load(Ordering::Relaxed);
        while live > peak {
            match self
                .peak
                .compare_exchange_weak(peak, live, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(observed) => peak = observed,
            }
        }
    }

    /// Simulates freeing `bytes` (saturating at zero).
    pub fn free(&self, bytes: u64) {
        let mut live = self.live.load(Ordering::Relaxed);
        loop {
            let next = live.saturating_sub(bytes);
            match self
                .live
                .compare_exchange_weak(live, next, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(observed) => live = observed,
            }
        }
    }
}

impl Default for ManualMemorySource {
    fn default() -> Self {
        Self::new()
    }
}

impl MemorySource for ManualMemorySource {
    fn live_bytes(&self) -> u64 {
        self.live.load(Ordering::Relaxed)
    }
    fn peak_bytes(&self) -> u64 {
        self.peak.load(Ordering::Relaxed)
    }
    fn total_allocated_bytes(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }
    fn allocation_events(&self) -> u64 {
        self.events.load(Ordering::Relaxed)
    }
    fn mark_peak_baseline(&self) {
        let live = self.live.load(Ordering::Relaxed);
        self.peak.store(live, Ordering::Relaxed);
    }
}

// ---------------------------------------------------------------------------
// Profiler
// ---------------------------------------------------------------------------

/// A single timed observation of a phase.
#[derive(Debug, Clone, Copy)]
pub struct Sample {
    /// Wall-clock duration of the phase.
    pub duration: Duration,
    /// Live-bytes delta over the phase (may be negative if memory was freed).
    pub alloc_delta: i64,
    /// Allocation events observed during the phase.
    pub alloc_events: u64,
    /// Peak live bytes observed during the phase.
    pub peak_live: u64,
}

/// Internal mutable state of a [`Profiler`].
#[derive(Default)]
struct ProfilerState {
    samples: BTreeMap<String, Vec<Sample>>,
    order: Vec<String>,
}

/// A phase-scoped, allocation-aware profiler.
///
/// Collect samples with [`measure`](Self::measure) / [`try_measure`](Self::try_measure),
/// then call [`analyze`](Self::analyze) to obtain a [`ProfileReport`].
pub struct Profiler {
    source: Arc<dyn MemorySource>,
    state: RefCell<ProfilerState>,
    thresholds: ProfileThresholds,
    started: Instant,
}

impl Profiler {
    /// Creates a profiler reading from the process-wide instrumented allocator.
    pub fn new() -> Self {
        Self::with_source(Arc::new(AllocatorSource::global()))
    }

    /// Creates a profiler reading from a custom [`MemorySource`].
    pub fn with_source(source: Arc<dyn MemorySource>) -> Self {
        Self {
            source,
            state: RefCell::new(ProfilerState::default()),
            thresholds: ProfileThresholds::default(),
            started: Instant::now(),
        }
    }

    /// Overrides the bottleneck-detection thresholds.
    pub fn with_thresholds(mut self, thresholds: ProfileThresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// Times `body`, attributing the result to `phase`, and returns its output.
    pub fn measure<T>(&self, phase: &str, body: impl FnOnce() -> T) -> T {
        let live_before = self.source.live_bytes();
        let events_before = self.source.allocation_events();
        self.source.mark_peak_baseline();

        let start = Instant::now();
        let output = body();
        let duration = start.elapsed();

        let live_after = self.source.live_bytes();
        let events_after = self.source.allocation_events();
        let peak_live = self.source.peak_bytes();

        let sample = Sample {
            duration,
            alloc_delta: live_after as i64 - live_before as i64,
            alloc_events: events_after.saturating_sub(events_before),
            peak_live,
        };
        self.push_sample(phase, sample);
        output
    }

    /// Times a fallible `body`, recording the sample regardless of outcome.
    pub fn try_measure<T, E>(
        &self,
        phase: &str,
        body: impl FnOnce() -> std::result::Result<T, E>,
    ) -> std::result::Result<T, E> {
        self.measure(phase, body)
    }

    /// Records a pre-computed sample for `phase`.
    pub fn record(&self, phase: &str, sample: Sample) {
        self.push_sample(phase, sample);
    }

    /// Appends a sample, preserving first-seen phase ordering.
    fn push_sample(&self, phase: &str, sample: Sample) {
        if let Ok(mut state) = self.state.try_borrow_mut() {
            if !state.samples.contains_key(phase) {
                state.order.push(phase.to_string());
            }
            state
                .samples
                .entry(phase.to_string())
                .or_default()
                .push(sample);
        }
    }

    /// Total number of samples recorded across all phases.
    pub fn sample_count(&self) -> usize {
        self.state
            .try_borrow()
            .map(|state| state.samples.values().map(Vec::len).sum())
            .unwrap_or(0)
    }

    /// Analyzes recorded samples into a structured [`ProfileReport`].
    pub fn analyze(&self) -> ProfileReport {
        let wall_clock_ms = duration_ms(self.started.elapsed());
        let Ok(state) = self.state.try_borrow() else {
            return ProfileReport::empty(wall_clock_ms);
        };

        let mut phases: Vec<PhaseProfile> = Vec::with_capacity(state.order.len());
        for name in &state.order {
            let Some(samples) = state.samples.get(name) else {
                continue;
            };
            if samples.is_empty() {
                continue;
            }
            let durations: Vec<Duration> = samples.iter().map(|s| s.duration).collect();
            let timing = DurationStats::from_durations(&durations);

            let alloc_sum: i64 = samples.iter().map(|s| s.alloc_delta).sum();
            let events_sum: u64 = samples.iter().map(|s| s.alloc_events).sum();
            let peak_alloc = samples.iter().map(|s| s.peak_live).max().unwrap_or(0);
            let count = samples.len() as i64;

            phases.push(PhaseProfile {
                name: name.clone(),
                samples: samples.len(),
                total_ms: timing.total_ms,
                share: 0.0,
                mean_alloc_bytes: alloc_sum / count.max(1),
                peak_alloc_bytes: peak_alloc,
                mean_alloc_events: events_sum / samples.len() as u64,
                timing,
            });
        }

        let accounted_ms: f64 = phases.iter().map(|p| p.total_ms).sum();
        if accounted_ms > 0.0 {
            for phase in &mut phases {
                phase.share = phase.total_ms / accounted_ms;
            }
        }

        let bottlenecks = self.detect_bottlenecks(&phases);
        let hints = self.suggest(&phases, &bottlenecks);

        ProfileReport {
            generated_at: chrono::Utc::now().to_rfc3339(),
            wall_clock_ms,
            accounted_ms,
            total_allocated_bytes: self.source.total_allocated_bytes(),
            peak_live_bytes: self.source.peak_bytes(),
            allocation_events: self.source.allocation_events(),
            phases,
            bottlenecks,
            hints,
        }
    }

    /// Detects bottlenecks across phases using the configured thresholds.
    fn detect_bottlenecks(&self, phases: &[PhaseProfile]) -> Vec<PhaseBottleneck> {
        let mut bottlenecks = Vec::new();
        let thresholds = &self.thresholds;

        for phase in phases {
            if phase.share >= thresholds.dominant_share {
                let severity = if phase.share >= 0.75 {
                    Severity::Critical
                } else if phase.share >= 0.6 {
                    Severity::High
                } else {
                    Severity::Medium
                };
                bottlenecks.push(PhaseBottleneck {
                    phase: phase.name.clone(),
                    kind: BottleneckKind::TimeShare,
                    severity,
                    detail: format!(
                        "Phase accounts for {:.1}% of accounted time",
                        phase.share * 100.0
                    ),
                });
            }

            if phase.timing.p95_ms >= thresholds.slow_p95_ms {
                bottlenecks.push(PhaseBottleneck {
                    phase: phase.name.clone(),
                    kind: BottleneckKind::Latency,
                    severity: severity_from_ratio(phase.timing.p95_ms / thresholds.slow_p95_ms),
                    detail: format!(
                        "p95 latency {:.2}ms exceeds {:.2}ms threshold",
                        phase.timing.p95_ms, thresholds.slow_p95_ms
                    ),
                });
            }

            let cv = phase.timing.coefficient_of_variation();
            if phase.samples >= 4 && cv >= thresholds.high_cv {
                bottlenecks.push(PhaseBottleneck {
                    phase: phase.name.clone(),
                    kind: BottleneckKind::Variance,
                    severity: severity_from_ratio(cv / thresholds.high_cv),
                    detail: format!("High latency variance (coefficient of variation {:.2})", cv),
                });
            }

            if phase.mean_alloc_bytes >= thresholds.heavy_alloc_bytes
                || phase.mean_alloc_events >= thresholds.heavy_alloc_events
            {
                bottlenecks.push(PhaseBottleneck {
                    phase: phase.name.clone(),
                    kind: BottleneckKind::Allocation,
                    severity: Severity::Medium,
                    detail: format!(
                        "Retains {} on average across {} allocations",
                        format_bytes(phase.mean_alloc_bytes.max(0) as u64),
                        phase.mean_alloc_events
                    ),
                });
            }
        }

        bottlenecks
    }

    /// Produces heuristic optimization hints from phases and detected bottlenecks.
    fn suggest(
        &self,
        phases: &[PhaseProfile],
        bottlenecks: &[PhaseBottleneck],
    ) -> Vec<OptimizationHint> {
        let mut hints: Vec<OptimizationHint> = Vec::new();

        for bottleneck in bottlenecks {
            let hint = match bottleneck.kind {
                BottleneckKind::TimeShare => OptimizationHint {
                    title: format!("Reduce time spent in '{}'", bottleneck.phase),
                    detail: "This phase dominates wall time. Consider caching its results, \
                             parallelizing independent work, or precomputing inputs."
                        .to_string(),
                    impact: HintImpact::High,
                    effort: HintEffort::Medium,
                },
                BottleneckKind::Latency => OptimizationHint {
                    title: format!("Lower tail latency in '{}'", bottleneck.phase),
                    detail: "High p95 latency indicates an expensive code path. Profile the \
                             slow inputs and replace quadratic logic or repeated parsing."
                        .to_string(),
                    impact: HintImpact::High,
                    effort: HintEffort::High,
                },
                BottleneckKind::Variance => OptimizationHint {
                    title: format!("Stabilize '{}' latency", bottleneck.phase),
                    detail: "Latency varies widely between runs. Warm caches, avoid cold I/O on \
                             the hot path, and reuse buffers to reduce jitter."
                        .to_string(),
                    impact: HintImpact::Medium,
                    effort: HintEffort::Medium,
                },
                BottleneckKind::Allocation => OptimizationHint {
                    title: format!("Cut allocations in '{}'", bottleneck.phase),
                    detail: "This phase allocates heavily. Reuse buffers, stream inputs, or \
                             switch to borrowed slices to relieve allocator pressure."
                        .to_string(),
                    impact: HintImpact::Medium,
                    effort: HintEffort::Medium,
                },
            };
            push_unique_hint(&mut hints, hint);
        }

        // Global heuristics independent of individual bottlenecks.
        if phases.len() >= 3 {
            push_unique_hint(
                &mut hints,
                OptimizationHint {
                    title: "Pipeline independent phases".to_string(),
                    detail: "Several phases were measured sequentially; overlapping I/O-bound and \
                             CPU-bound phases can shorten end-to-end latency."
                        .to_string(),
                    impact: HintImpact::Medium,
                    effort: HintEffort::High,
                },
            );
        }

        if self.source.allocation_events() >= self.thresholds.heavy_alloc_events {
            push_unique_hint(
                &mut hints,
                OptimizationHint {
                    title: "Reduce overall allocation count".to_string(),
                    detail: "A large number of allocations were observed. Prefer arena/bump \
                             allocation or buffer reuse for hot loops."
                        .to_string(),
                    impact: HintImpact::Medium,
                    effort: HintEffort::Medium,
                },
            );
        }

        hints
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Inserts a hint unless one with the same title already exists.
fn push_unique_hint(hints: &mut Vec<OptimizationHint>, hint: OptimizationHint) {
    if !hints.iter().any(|existing| existing.title == hint.title) {
        hints.push(hint);
    }
}

/// Maps a ratio above 1.0 onto a [`Severity`].
fn severity_from_ratio(ratio: f64) -> Severity {
    if ratio >= 4.0 {
        Severity::Critical
    } else if ratio >= 2.0 {
        Severity::High
    } else if ratio >= 1.0 {
        Severity::Medium
    } else {
        Severity::Low
    }
}

/// Formats a byte count using binary units.
fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.2} {}", value, UNITS[unit])
    }
}

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

/// Summary statistics for a set of phase durations, in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DurationStats {
    /// Number of samples.
    pub count: usize,
    /// Minimum duration.
    pub min_ms: f64,
    /// Maximum duration.
    pub max_ms: f64,
    /// Arithmetic mean.
    pub mean_ms: f64,
    /// Population standard deviation.
    pub stddev_ms: f64,
    /// 50th percentile (median).
    pub p50_ms: f64,
    /// 90th percentile.
    pub p90_ms: f64,
    /// 95th percentile.
    pub p95_ms: f64,
    /// 99th percentile.
    pub p99_ms: f64,
    /// Sum of all durations.
    pub total_ms: f64,
}

impl DurationStats {
    /// Computes statistics from a slice of durations.
    ///
    /// Percentiles use linear interpolation between the two closest ranks (the
    /// method used by NumPy's default `linear` interpolation and most
    /// observability tools).
    pub fn from_durations(durations: &[Duration]) -> Self {
        if durations.is_empty() {
            return Self::zeroed();
        }

        let mut nanos: Vec<u128> = durations.iter().map(Duration::as_nanos).collect();
        nanos.sort_unstable();
        let count = nanos.len();

        let total_ns: f64 = nanos.iter().map(|&n| n as f64).sum();
        let mean_ns = total_ns / count as f64;
        let variance_ns = nanos
            .iter()
            .map(|&n| {
                let diff = n as f64 - mean_ns;
                diff * diff
            })
            .sum::<f64>()
            / count as f64;
        let stddev_ns = variance_ns.sqrt();

        let to_ms = |ns: f64| ns / 1_000_000.0;

        Self {
            count,
            min_ms: to_ms(nanos[0] as f64),
            max_ms: to_ms(nanos[count - 1] as f64),
            mean_ms: to_ms(mean_ns),
            stddev_ms: to_ms(stddev_ns),
            p50_ms: to_ms(percentile_ns(&nanos, 0.50)),
            p90_ms: to_ms(percentile_ns(&nanos, 0.90)),
            p95_ms: to_ms(percentile_ns(&nanos, 0.95)),
            p99_ms: to_ms(percentile_ns(&nanos, 0.99)),
            total_ms: to_ms(total_ns),
        }
    }

    /// All-zero statistics for an empty sample set.
    fn zeroed() -> Self {
        Self {
            count: 0,
            min_ms: 0.0,
            max_ms: 0.0,
            mean_ms: 0.0,
            stddev_ms: 0.0,
            p50_ms: 0.0,
            p90_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            total_ms: 0.0,
        }
    }

    /// Coefficient of variation (`stddev / mean`); zero when the mean is zero.
    pub fn coefficient_of_variation(&self) -> f64 {
        if self.mean_ms > 0.0 {
            self.stddev_ms / self.mean_ms
        } else {
            0.0
        }
    }
}

/// Linear-interpolation percentile over a sorted slice of nanosecond samples.
fn percentile_ns(sorted: &[u128], quantile: f64) -> f64 {
    let count = sorted.len();
    if count == 0 {
        return 0.0;
    }
    if count == 1 {
        return sorted[0] as f64;
    }
    let clamped = quantile.clamp(0.0, 1.0);
    let rank = clamped * (count - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let lower_value = sorted[lower] as f64;
    let upper_value = sorted[upper] as f64;
    let fraction = rank - lower as f64;
    lower_value + (upper_value - lower_value) * fraction
}

// ---------------------------------------------------------------------------
// Report types
// ---------------------------------------------------------------------------

/// Per-phase aggregated profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseProfile {
    /// Phase name.
    pub name: String,
    /// Number of samples collected.
    pub samples: usize,
    /// Total time across all samples.
    pub total_ms: f64,
    /// Fraction of accounted time spent in this phase.
    pub share: f64,
    /// Mean live-bytes delta per sample.
    pub mean_alloc_bytes: i64,
    /// Maximum peak live bytes observed across samples.
    pub peak_alloc_bytes: u64,
    /// Mean allocation events per sample.
    pub mean_alloc_events: u64,
    /// Timing statistics (kept last for clean TOML serialization).
    pub timing: DurationStats,
}

/// Classification of a detected bottleneck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottleneckKind {
    /// Dominates the share of accounted wall time.
    TimeShare,
    /// High tail latency.
    Latency,
    /// High run-to-run variance.
    Variance,
    /// Heavy allocation pressure.
    Allocation,
}

/// Severity of a bottleneck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Minor; informational.
    Low,
    /// Worth attention.
    Medium,
    /// Significant.
    High,
    /// Severe; address first.
    Critical,
}

/// A detected performance bottleneck attributed to a phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhaseBottleneck {
    /// Phase the bottleneck belongs to.
    pub phase: String,
    /// Severity of the bottleneck.
    pub severity: Severity,
    /// Classification of the bottleneck.
    pub kind: BottleneckKind,
    /// Human-readable explanation.
    pub detail: String,
}

/// Expected impact of an optimization hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HintImpact {
    /// Small win.
    Low,
    /// Moderate win.
    Medium,
    /// Large win.
    High,
}

/// Estimated implementation effort of an optimization hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HintEffort {
    /// Quick change.
    Low,
    /// Moderate change.
    Medium,
    /// Substantial change.
    High,
}

/// A heuristic optimization suggestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationHint {
    /// Short title.
    pub title: String,
    /// Estimated impact.
    pub impact: HintImpact,
    /// Estimated effort.
    pub effort: HintEffort,
    /// Detailed guidance.
    pub detail: String,
}

/// A complete profiling report ready for rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileReport {
    /// RFC 3339 timestamp of report generation.
    pub generated_at: String,
    /// Wall-clock time since the profiler was created.
    pub wall_clock_ms: f64,
    /// Sum of all measured phase time.
    pub accounted_ms: f64,
    /// Process cumulative bytes allocated (from the memory source).
    pub total_allocated_bytes: u64,
    /// Process peak live bytes (from the memory source).
    pub peak_live_bytes: u64,
    /// Process allocation events (from the memory source).
    pub allocation_events: u64,
    /// Per-phase profiles.
    pub phases: Vec<PhaseProfile>,
    /// Detected bottlenecks.
    pub bottlenecks: Vec<PhaseBottleneck>,
    /// Optimization hints.
    pub hints: Vec<OptimizationHint>,
}

impl ProfileReport {
    /// An empty report with only the wall clock populated.
    fn empty(wall_clock_ms: f64) -> Self {
        Self {
            generated_at: chrono::Utc::now().to_rfc3339(),
            wall_clock_ms,
            accounted_ms: 0.0,
            total_allocated_bytes: 0,
            peak_live_bytes: 0,
            allocation_events: 0,
            phases: Vec::new(),
            bottlenecks: Vec::new(),
            hints: Vec::new(),
        }
    }

    /// Renders the report in the requested [`OutputFormat`].
    pub fn render(&self, format: &OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
            OutputFormat::Yaml => Ok(serde_yaml::to_string(self)?),
            OutputFormat::Toml => Ok(toml::to_string_pretty(self)?),
            OutputFormat::Csv => self.render_csv(),
            OutputFormat::Table => Ok(self.render_table()),
            OutputFormat::Html => Ok(self.render_html()),
            OutputFormat::Text => Ok(self.render_text()),
        }
    }

    /// Renders a plain-text report.
    fn render_text(&self) -> String {
        let mut out = String::new();
        out.push_str("Performance Profile\n");
        out.push_str("===================\n");
        out.push_str(&format!("Generated:        {}\n", self.generated_at));
        out.push_str(&format!("Wall clock:       {:.2} ms\n", self.wall_clock_ms));
        out.push_str(&format!("Accounted time:   {:.2} ms\n", self.accounted_ms));
        out.push_str(&format!(
            "Peak live memory: {}\n",
            format_bytes(self.peak_live_bytes)
        ));
        out.push_str(&format!(
            "Total allocated:  {} ({} events)\n\n",
            format_bytes(self.total_allocated_bytes),
            self.allocation_events
        ));

        out.push_str("Phases:\n");
        for phase in &self.phases {
            out.push_str(&format!(
                "  {:<16} n={:<4} mean={:>8.3}ms  p95={:>8.3}ms  p99={:>8.3}ms  share={:>5.1}%  alloc={}\n",
                phase.name,
                phase.samples,
                phase.timing.mean_ms,
                phase.timing.p95_ms,
                phase.timing.p99_ms,
                phase.share * 100.0,
                format_bytes(phase.mean_alloc_bytes.max(0) as u64),
            ));
        }

        if self.bottlenecks.is_empty() {
            out.push_str("\nBottlenecks: none detected\n");
        } else {
            out.push_str("\nBottlenecks:\n");
            for bottleneck in &self.bottlenecks {
                out.push_str(&format!(
                    "  [{:?}/{:?}] {}: {}\n",
                    bottleneck.severity, bottleneck.kind, bottleneck.phase, bottleneck.detail
                ));
            }
        }

        if !self.hints.is_empty() {
            out.push_str("\nOptimization hints:\n");
            for hint in &self.hints {
                out.push_str(&format!(
                    "  ({:?} impact / {:?} effort) {}\n      {}\n",
                    hint.impact, hint.effort, hint.title, hint.detail
                ));
            }
        }

        out
    }

    /// Renders the phase table using `comfy-table`.
    fn render_table(&self) -> String {
        use comfy_table::{Cell, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Phase"),
                Cell::new("Samples"),
                Cell::new("Mean (ms)"),
                Cell::new("p95 (ms)"),
                Cell::new("p99 (ms)"),
                Cell::new("Share %"),
                Cell::new("Mean alloc"),
            ]);

        for phase in &self.phases {
            table.add_row(vec![
                Cell::new(&phase.name),
                Cell::new(phase.samples),
                Cell::new(format!("{:.3}", phase.timing.mean_ms)),
                Cell::new(format!("{:.3}", phase.timing.p95_ms)),
                Cell::new(format!("{:.3}", phase.timing.p99_ms)),
                Cell::new(format!("{:.1}", phase.share * 100.0)),
                Cell::new(format_bytes(phase.mean_alloc_bytes.max(0) as u64)),
            ]);
        }

        table.to_string()
    }

    /// Renders the phase table as CSV.
    fn render_csv(&self) -> Result<String> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer.write_record([
            "phase",
            "samples",
            "mean_ms",
            "p50_ms",
            "p95_ms",
            "p99_ms",
            "share",
            "mean_alloc_bytes",
            "peak_alloc_bytes",
        ])?;
        for phase in &self.phases {
            writer.write_record([
                phase.name.as_str(),
                &phase.samples.to_string(),
                &format!("{:.6}", phase.timing.mean_ms),
                &format!("{:.6}", phase.timing.p50_ms),
                &format!("{:.6}", phase.timing.p95_ms),
                &format!("{:.6}", phase.timing.p99_ms),
                &format!("{:.6}", phase.share),
                &phase.mean_alloc_bytes.to_string(),
                &phase.peak_alloc_bytes.to_string(),
            ])?;
        }
        writer.flush()?;
        let bytes = writer
            .into_inner()
            .map_err(|error| anyhow::anyhow!("Failed to finalize CSV writer: {error}"))?;
        String::from_utf8(bytes)
            .map_err(|error| anyhow::anyhow!("Profile CSV was not valid UTF-8: {error}"))
    }

    /// Renders a simple self-contained HTML report.
    fn render_html(&self) -> String {
        let mut out = String::new();
        out.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"UTF-8\">\n");
        out.push_str("<title>Legalis Performance Profile</title>\n");
        out.push_str("<style>body{font-family:Arial,sans-serif;margin:20px;}");
        out.push_str("table{border-collapse:collapse;width:100%;}");
        out.push_str("th,td{border:1px solid #ddd;padding:8px;text-align:left;}");
        out.push_str("th{background:#2c3e50;color:#fff;}</style></head><body>\n");
        out.push_str("<h1>Performance Profile</h1>\n");
        out.push_str(&format!(
            "<p>Generated {} &mdash; wall clock {:.2} ms, peak live {}.</p>\n",
            self.generated_at,
            self.wall_clock_ms,
            format_bytes(self.peak_live_bytes)
        ));
        out.push_str("<table><tr><th>Phase</th><th>Samples</th><th>Mean (ms)</th>");
        out.push_str(
            "<th>p95 (ms)</th><th>p99 (ms)</th><th>Share %</th><th>Mean alloc</th></tr>\n",
        );
        for phase in &self.phases {
            out.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{:.3}</td><td>{:.3}</td><td>{:.3}</td><td>{:.1}</td><td>{}</td></tr>\n",
                phase.name,
                phase.samples,
                phase.timing.mean_ms,
                phase.timing.p95_ms,
                phase.timing.p99_ms,
                phase.share * 100.0,
                format_bytes(phase.mean_alloc_bytes.max(0) as u64),
            ));
        }
        out.push_str("</table>\n");

        if !self.bottlenecks.is_empty() {
            out.push_str("<h2>Bottlenecks</h2>\n<ul>\n");
            for bottleneck in &self.bottlenecks {
                out.push_str(&format!(
                    "<li><strong>{:?}/{:?}</strong> {}: {}</li>\n",
                    bottleneck.severity, bottleneck.kind, bottleneck.phase, bottleneck.detail
                ));
            }
            out.push_str("</ul>\n");
        }

        if !self.hints.is_empty() {
            out.push_str("<h2>Optimization Hints</h2>\n<ul>\n");
            for hint in &self.hints {
                out.push_str(&format!(
                    "<li><strong>{}</strong> ({:?} impact / {:?} effort): {}</li>\n",
                    hint.title, hint.impact, hint.effort, hint.detail
                ));
            }
            out.push_str("</ul>\n");
        }

        out.push_str("</body></html>\n");
        out
    }
}

/// Thresholds that govern bottleneck detection.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProfileThresholds {
    /// Minimum share of accounted time for a phase to be a time-share bottleneck.
    pub dominant_share: f64,
    /// Minimum p95 latency (ms) for a phase to be a latency bottleneck.
    pub slow_p95_ms: f64,
    /// Minimum coefficient of variation for a variance bottleneck.
    pub high_cv: f64,
    /// Minimum mean live-bytes delta for an allocation bottleneck.
    pub heavy_alloc_bytes: i64,
    /// Minimum allocation events for an allocation/global bottleneck.
    pub heavy_alloc_events: u64,
}

impl Default for ProfileThresholds {
    fn default() -> Self {
        Self {
            dominant_share: 0.40,
            slow_p95_ms: 250.0,
            high_cv: 0.50,
            heavy_alloc_bytes: 8 * 1024 * 1024,
            heavy_alloc_events: 100_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(value: u64) -> Duration {
        Duration::from_millis(value)
    }

    #[test]
    fn tracking_allocator_accounts_allocations() {
        let allocator = TrackingAllocator::new(System);
        let layout = Layout::from_size_align(1024, 8).expect("valid layout");

        // SAFETY: layout is valid; pointer is freed below.
        let ptr = unsafe { allocator.alloc(layout) };
        assert!(!ptr.is_null());
        assert_eq!(allocator.live(), 1024);
        assert_eq!(allocator.peak(), 1024);
        assert_eq!(allocator.total_allocated(), 1024);
        assert_eq!(allocator.allocation_count(), 1);

        // SAFETY: ptr came from the same allocator and layout.
        unsafe { allocator.dealloc(ptr, layout) };
        assert_eq!(allocator.live(), 0);
        assert_eq!(allocator.peak(), 1024, "peak is retained after free");
        assert_eq!(allocator.deallocation_count(), 1);
    }

    #[test]
    fn tracking_allocator_reset_peak_follows_live() {
        let allocator = TrackingAllocator::new(System);
        let layout = Layout::from_size_align(512, 8).expect("valid layout");
        let ptr = unsafe { allocator.alloc(layout) };
        unsafe { allocator.dealloc(ptr, layout) };
        assert_eq!(allocator.peak(), 512);
        allocator.reset_peak();
        assert_eq!(allocator.peak(), 0, "reset drops peak to current live");
    }

    #[test]
    fn tracking_allocator_realloc_growth_and_shrink() {
        let allocator = TrackingAllocator::new(System);
        let small = Layout::from_size_align(64, 8).expect("valid layout");
        let ptr = unsafe { allocator.alloc(small) };
        assert_eq!(allocator.live(), 64);

        let grown = unsafe { allocator.realloc(ptr, small, 256) };
        assert!(!grown.is_null());
        assert_eq!(allocator.live(), 256);
        assert_eq!(allocator.peak(), 256);

        let shrunk_layout = Layout::from_size_align(256, 8).expect("valid layout");
        let shrunk = unsafe { allocator.realloc(grown, shrunk_layout, 32) };
        assert_eq!(allocator.live(), 32);

        let final_layout = Layout::from_size_align(32, 8).expect("valid layout");
        unsafe { allocator.dealloc(shrunk, final_layout) };
        assert_eq!(allocator.live(), 0);
    }

    #[test]
    fn duration_stats_basic_aggregates() {
        let stats = DurationStats::from_durations(&[ms(10), ms(20), ms(30)]);
        assert_eq!(stats.count, 3);
        assert!((stats.min_ms - 10.0).abs() < 1e-6);
        assert!((stats.max_ms - 30.0).abs() < 1e-6);
        assert!((stats.mean_ms - 20.0).abs() < 1e-6);
        assert!((stats.total_ms - 60.0).abs() < 1e-6);
        assert!((stats.p50_ms - 20.0).abs() < 1e-6);
    }

    #[test]
    fn duration_stats_percentiles_interpolate() {
        let durations: Vec<Duration> = (1..=100).map(ms).collect();
        let stats = DurationStats::from_durations(&durations);
        // Linear interpolation over 1..=100: p50 = 50.5, p90 = 90.1, p99 = 99.01.
        assert!(
            (stats.p50_ms - 50.5).abs() < 1e-3,
            "p50 was {}",
            stats.p50_ms
        );
        assert!(
            (stats.p90_ms - 90.1).abs() < 1e-3,
            "p90 was {}",
            stats.p90_ms
        );
        assert!(
            (stats.p99_ms - 99.01).abs() < 1e-3,
            "p99 was {}",
            stats.p99_ms
        );
    }

    #[test]
    fn duration_stats_empty_and_single() {
        let empty = DurationStats::from_durations(&[]);
        assert_eq!(empty.count, 0);
        assert_eq!(empty.p99_ms, 0.0);

        let single = DurationStats::from_durations(&[ms(42)]);
        assert_eq!(single.count, 1);
        assert!((single.p50_ms - 42.0).abs() < 1e-6);
        assert!((single.p99_ms - 42.0).abs() < 1e-6);
        assert_eq!(single.stddev_ms, 0.0);
    }

    #[test]
    fn percentiles_are_monotonic() {
        let durations: Vec<Duration> = [5u64, 1, 9, 3, 7, 2, 8, 4, 6]
            .iter()
            .map(|&n| ms(n))
            .collect();
        let stats = DurationStats::from_durations(&durations);
        assert!(stats.p50_ms <= stats.p90_ms);
        assert!(stats.p90_ms <= stats.p95_ms);
        assert!(stats.p95_ms <= stats.p99_ms);
        assert!(stats.p99_ms <= stats.max_ms + 1e-9);
    }

    #[test]
    fn manual_memory_source_tracks_peak() {
        let source = ManualMemorySource::new();
        source.allocate(1000);
        source.allocate(500);
        assert_eq!(source.live_bytes(), 1500);
        assert_eq!(source.peak_bytes(), 1500);
        source.free(800);
        assert_eq!(source.live_bytes(), 700);
        assert_eq!(source.peak_bytes(), 1500);
        assert_eq!(source.allocation_events(), 2);
        source.mark_peak_baseline();
        assert_eq!(source.peak_bytes(), 700);
    }

    #[test]
    fn profiler_measure_records_phase_samples() {
        let source = Arc::new(ManualMemorySource::new());
        let profiler = Profiler::with_source(source.clone());

        for _ in 0..3 {
            profiler.measure("parse", || {
                source.allocate(2048);
                std::thread::sleep(Duration::from_millis(1));
            });
        }
        profiler.measure("verify", || {
            std::thread::sleep(Duration::from_millis(1));
        });

        assert_eq!(profiler.sample_count(), 4);
        let report = profiler.analyze();
        assert_eq!(report.phases.len(), 2);
        let total_share: f64 = report.phases.iter().map(|p| p.share).sum();
        assert!((total_share - 1.0).abs() < 1e-6, "shares should sum to 1");
        let parse = report
            .phases
            .iter()
            .find(|p| p.name == "parse")
            .expect("parse phase present");
        assert_eq!(parse.samples, 3);
        assert!(parse.mean_alloc_bytes > 0);
    }

    #[test]
    fn profiler_try_measure_propagates_error() {
        let profiler = Profiler::with_source(Arc::new(ManualMemorySource::new()));
        let result: std::result::Result<(), &str> =
            profiler.try_measure("io", || Err("disk offline"));
        assert_eq!(result, Err("disk offline"));
        assert_eq!(profiler.sample_count(), 1, "sample recorded despite error");
    }

    #[test]
    fn bottleneck_detection_flags_dominant_phase() {
        let source = Arc::new(ManualMemorySource::new());
        let profiler = Profiler::with_source(source.clone());
        profiler.measure("fast", || std::thread::sleep(Duration::from_millis(1)));
        profiler.measure("slow", || std::thread::sleep(Duration::from_millis(40)));

        let report = profiler.analyze();
        assert!(
            report
                .bottlenecks
                .iter()
                .any(|b| b.phase == "slow" && b.kind == BottleneckKind::TimeShare),
            "expected a time-share bottleneck for the slow phase: {:?}",
            report.bottlenecks
        );
        assert!(
            !report.hints.is_empty(),
            "hints should accompany bottlenecks"
        );
    }

    #[test]
    fn bottleneck_detection_flags_heavy_allocation() {
        let source = Arc::new(ManualMemorySource::new());
        let thresholds = ProfileThresholds {
            heavy_alloc_bytes: 1024,
            ..ProfileThresholds::default()
        };
        let profiler = Profiler::with_source(source.clone()).with_thresholds(thresholds);
        profiler.measure("load", || {
            source.allocate(64 * 1024);
        });
        let report = profiler.analyze();
        assert!(
            report
                .bottlenecks
                .iter()
                .any(|b| b.kind == BottleneckKind::Allocation),
            "expected an allocation bottleneck: {:?}",
            report.bottlenecks
        );
    }

    #[test]
    fn report_renders_in_all_formats() {
        let source = Arc::new(ManualMemorySource::new());
        let profiler = Profiler::with_source(source.clone());
        profiler.measure("parse", || {
            source.allocate(4096);
            std::thread::sleep(Duration::from_millis(2));
        });
        let report = profiler.analyze();

        let json = report.render(&OutputFormat::Json).expect("json render");
        let parsed: ProfileReport = serde_json::from_str(&json).expect("round-trips through json");
        assert_eq!(parsed.phases.len(), report.phases.len());

        for format in [
            OutputFormat::Yaml,
            OutputFormat::Toml,
            OutputFormat::Csv,
            OutputFormat::Table,
            OutputFormat::Html,
            OutputFormat::Text,
        ] {
            let rendered = report.render(&format).expect("render succeeds");
            assert!(
                rendered.contains("parse"),
                "format {:?} should mention the parse phase",
                format
            );
        }
    }

    #[test]
    fn format_bytes_uses_binary_units() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(2048), "2.00 KiB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.00 MiB");
    }
}
