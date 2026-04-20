use super::*;

/// SLA metric type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlaMetric {
    /// Time to first response
    TimeToFirstResponse,
    /// Time to approval
    TimeToApproval,
    /// Time to completion
    TimeToCompletion,
    /// Custom metric
    Custom(String),
}

/// SLA definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaDefinition {
    /// SLA ID
    pub sla_id: Uuid,
    /// SLA name
    pub name: String,
    /// Metric being tracked
    pub metric: SlaMetric,
    /// Target duration in seconds
    pub target_seconds: i64,
    /// Warning threshold (percentage of target)
    pub warning_threshold: f64,
}

impl SlaDefinition {
    /// Creates a new SLA definition.
    pub fn new(name: impl Into<String>, metric: SlaMetric, target_seconds: i64) -> Self {
        Self {
            sla_id: Uuid::new_v4(),
            name: name.into(),
            metric,
            target_seconds,
            warning_threshold: 0.8, // 80% of target
        }
    }

    /// Sets warning threshold.
    pub fn with_warning_threshold(mut self, threshold: f64) -> Self {
        self.warning_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Gets target duration.
    pub fn target_duration(&self) -> chrono::Duration {
        chrono::Duration::seconds(self.target_seconds)
    }

    /// Gets warning duration.
    pub fn warning_duration(&self) -> chrono::Duration {
        chrono::Duration::seconds((self.target_seconds as f64 * self.warning_threshold) as i64)
    }
}

/// SLA status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlaStatus {
    /// Met the SLA
    Met,
    /// Warning - approaching SLA breach
    Warning,
    /// Breached the SLA
    Breached,
}

/// SLA measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaMeasurement {
    /// Measurement ID
    pub measurement_id: Uuid,
    /// SLA definition ID
    pub sla_id: Uuid,
    /// Related entity ID
    pub entity_id: String,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    /// Actual duration in seconds
    pub duration_seconds: Option<i64>,
    /// SLA status
    pub status: SlaStatus,
}

impl SlaMeasurement {
    /// Creates a new SLA measurement.
    pub fn new(sla_id: Uuid, entity_id: impl Into<String>) -> Self {
        Self {
            measurement_id: Uuid::new_v4(),
            sla_id,
            entity_id: entity_id.into(),
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: None,
            status: SlaStatus::Met,
        }
    }

    /// Completes the measurement.
    pub fn complete(&mut self, sla: &SlaDefinition) {
        self.end_time = Some(Utc::now());
        let duration = self
            .end_time
            .expect("invariant: end_time was just set to Some")
            - self.start_time;
        self.duration_seconds = Some(duration.num_seconds());

        // Determine status
        if duration > sla.target_duration() {
            self.status = SlaStatus::Breached;
        } else if duration > sla.warning_duration() {
            self.status = SlaStatus::Warning;
        } else {
            self.status = SlaStatus::Met;
        }
    }

    /// Checks current status against SLA.
    pub fn check_status(&mut self, sla: &SlaDefinition) -> SlaStatus {
        if self.end_time.is_some() {
            return self.status;
        }

        let elapsed = Utc::now() - self.start_time;
        if elapsed > sla.target_duration() {
            self.status = SlaStatus::Breached;
        } else if elapsed > sla.warning_duration() {
            self.status = SlaStatus::Warning;
        } else {
            self.status = SlaStatus::Met;
        }
        self.status
    }
}

/// SLA tracker.
#[derive(Debug)]
pub struct SlaTracker {
    definitions: HashMap<Uuid, SlaDefinition>,
    measurements: Vec<SlaMeasurement>,
}

impl SlaTracker {
    /// Creates a new SLA tracker.
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            measurements: Vec::new(),
        }
    }

    /// Adds an SLA definition.
    pub fn add_definition(&mut self, definition: SlaDefinition) -> Uuid {
        let id = definition.sla_id;
        self.definitions.insert(id, definition);
        id
    }

    /// Starts tracking an SLA.
    pub fn start_tracking(&mut self, sla_id: Uuid, entity_id: impl Into<String>) -> Uuid {
        let measurement = SlaMeasurement::new(sla_id, entity_id);
        let id = measurement.measurement_id;
        self.measurements.push(measurement);
        id
    }

    /// Completes an SLA measurement.
    pub fn complete_measurement(&mut self, measurement_id: Uuid) -> Result<SlaStatus, String> {
        let measurement = self
            .measurements
            .iter_mut()
            .find(|m| m.measurement_id == measurement_id)
            .ok_or_else(|| "Measurement not found".to_string())?;

        let sla = self
            .definitions
            .get(&measurement.sla_id)
            .ok_or_else(|| "SLA definition not found".to_string())?;

        measurement.complete(sla);
        Ok(measurement.status)
    }

    /// Gets measurements in warning or breach status.
    pub fn at_risk_measurements(&mut self) -> Vec<&mut SlaMeasurement> {
        // First update all statuses
        for m in &mut self.measurements {
            if let Some(sla) = self.definitions.get(&m.sla_id) {
                m.check_status(sla);
            }
        }

        // Then filter based on updated status
        self.measurements
            .iter_mut()
            .filter(|m| m.status == SlaStatus::Warning || m.status == SlaStatus::Breached)
            .collect()
    }

    /// Gets completion rate for an SLA.
    pub fn completion_rate(&self, sla_id: Uuid) -> f64 {
        let total: Vec<_> = self
            .measurements
            .iter()
            .filter(|m| m.sla_id == sla_id && m.end_time.is_some())
            .collect();

        if total.is_empty() {
            return 1.0;
        }

        let met_count = total.iter().filter(|m| m.status == SlaStatus::Met).count();

        met_count as f64 / total.len() as f64
    }
}

impl Default for SlaTracker {
    fn default() -> Self {
        Self::new()
    }
}
