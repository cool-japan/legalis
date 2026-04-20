use super::*;

/// Workflow status for a statute change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Draft - not yet submitted for approval
    Draft,
    /// Pending approval
    PendingApproval,
    /// Approved and ready to apply
    Approved,
    /// Rejected with reason
    Rejected,
    /// Cancelled by submitter
    Cancelled,
}

/// Type of change being proposed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Creating a new statute
    Create,
    /// Updating an existing statute
    Update { statute_id: String },
    /// Deleting a statute
    Delete { statute_id: String },
    /// Changing status
    StatusChange {
        statute_id: String,
        new_status: StatuteStatus,
    },
    /// Bulk operation
    Bulk { operation_count: usize },
}

/// An approval request for a statute change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// Unique request ID
    pub request_id: Uuid,
    /// Type of change
    pub change_type: ChangeType,
    /// Submitter user ID
    pub submitter: String,
    /// Workflow status
    pub status: WorkflowStatus,
    /// Requested change data (JSON)
    pub change_data: String,
    /// Justification for the change
    pub justification: Option<String>,
    /// Approvers assigned
    pub approvers: Vec<String>,
    /// Approval responses
    pub responses: Vec<ApprovalResponse>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Due date for approval
    pub due_date: Option<DateTime<Utc>>,
}

impl ApprovalRequest {
    /// Creates a new approval request.
    pub fn new(
        change_type: ChangeType,
        submitter: impl Into<String>,
        change_data: impl Into<String>,
    ) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            change_type,
            submitter: submitter.into(),
            status: WorkflowStatus::Draft,
            change_data: change_data.into(),
            justification: None,
            approvers: Vec::new(),
            responses: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
        }
    }

    /// Sets the justification.
    pub fn with_justification(mut self, justification: impl Into<String>) -> Self {
        self.justification = Some(justification.into());
        self
    }

    /// Adds an approver.
    pub fn with_approver(mut self, approver: impl Into<String>) -> Self {
        self.approvers.push(approver.into());
        self
    }

    /// Sets the due date.
    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Submits the request for approval.
    pub fn submit(&mut self) {
        self.status = WorkflowStatus::PendingApproval;
        self.updated_at = Utc::now();
    }

    /// Adds an approval response.
    pub fn add_response(&mut self, response: ApprovalResponse) {
        self.responses.push(response);
        self.updated_at = Utc::now();
    }

    /// Checks if the request is approved (all approvers approved).
    pub fn is_approved(&self) -> bool {
        if self.approvers.is_empty() {
            return false;
        }
        let approved_count = self
            .responses
            .iter()
            .filter(|r| r.decision == ApprovalDecision::Approved)
            .count();
        approved_count >= self.approvers.len()
    }

    /// Checks if the request is rejected (any approver rejected).
    pub fn is_rejected(&self) -> bool {
        self.responses
            .iter()
            .any(|r| r.decision == ApprovalDecision::Rejected)
    }

    /// Checks if the request is overdue.
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            Utc::now() > due && self.status == WorkflowStatus::PendingApproval
        } else {
            false
        }
    }
}

/// Approval decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Needs more information
    NeedsInfo,
}

/// An approval response from an approver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    /// Approver user ID
    pub approver: String,
    /// Decision
    pub decision: ApprovalDecision,
    /// Comments
    pub comments: Option<String>,
    /// Response timestamp
    pub responded_at: DateTime<Utc>,
}

impl ApprovalResponse {
    /// Creates a new approval response.
    pub fn new(approver: impl Into<String>, decision: ApprovalDecision) -> Self {
        Self {
            approver: approver.into(),
            decision,
            comments: None,
            responded_at: Utc::now(),
        }
    }

    /// Sets comments.
    pub fn with_comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }
}

/// Type alias for auto-approval rule functions.
pub type AutoApproveRule = Box<dyn Fn(&ApprovalRequest) -> bool + Send + Sync>;

/// Workflow manager for approval requests.
pub struct WorkflowManager {
    requests: HashMap<Uuid, ApprovalRequest>,
    /// Auto-approval rules
    auto_approve_rules: Vec<AutoApproveRule>,
}

impl std::fmt::Debug for WorkflowManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowManager")
            .field("requests", &self.requests)
            .field(
                "auto_approve_rules",
                &format!("<{} rules>", self.auto_approve_rules.len()),
            )
            .finish()
    }
}

impl WorkflowManager {
    /// Creates a new workflow manager.
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            auto_approve_rules: Vec::new(),
        }
    }

    /// Submits a new approval request.
    pub fn submit_request(&mut self, mut request: ApprovalRequest) -> Uuid {
        request.submit();
        let id = request.request_id;

        // Check auto-approval rules
        for rule in &self.auto_approve_rules {
            if rule(&request) {
                request.status = WorkflowStatus::Approved;
                break;
            }
        }

        self.requests.insert(id, request);
        id
    }

    /// Gets a request by ID.
    pub fn get_request(&self, request_id: Uuid) -> Option<&ApprovalRequest> {
        self.requests.get(&request_id)
    }

    /// Adds a response to a request.
    pub fn add_response(
        &mut self,
        request_id: Uuid,
        response: ApprovalResponse,
    ) -> Result<(), String> {
        let request = self
            .requests
            .get_mut(&request_id)
            .ok_or_else(|| "Request not found".to_string())?;

        if request.status != WorkflowStatus::PendingApproval {
            return Err("Request is not pending approval".to_string());
        }

        request.add_response(response);

        // Update status based on responses
        if request.is_rejected() {
            request.status = WorkflowStatus::Rejected;
        } else if request.is_approved() {
            request.status = WorkflowStatus::Approved;
        }

        Ok(())
    }

    /// Gets pending requests.
    pub fn pending_requests(&self) -> Vec<&ApprovalRequest> {
        self.requests
            .values()
            .filter(|r| r.status == WorkflowStatus::PendingApproval)
            .collect()
    }

    /// Gets overdue requests.
    pub fn overdue_requests(&self) -> Vec<&ApprovalRequest> {
        self.requests.values().filter(|r| r.is_overdue()).collect()
    }

    /// Gets requests for a specific approver.
    pub fn requests_for_approver(&self, approver: &str) -> Vec<&ApprovalRequest> {
        self.requests
            .values()
            .filter(|r| {
                r.approvers.contains(&approver.to_string())
                    && r.status == WorkflowStatus::PendingApproval
            })
            .collect()
    }
}

impl Default for WorkflowManager {
    fn default() -> Self {
        Self::new()
    }
}
