use super::*;

/// Task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Not yet started
    NotStarted,
    /// In progress
    InProgress,
    /// Blocked
    Blocked,
    /// Completed
    Completed,
    /// Cancelled
    Cancelled,
}

/// Review task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewTask {
    /// Task ID
    pub task_id: Uuid,
    /// Task title
    pub title: String,
    /// Task description
    pub description: Option<String>,
    /// Assigned to user ID
    pub assigned_to: String,
    /// Assigned by user ID
    pub assigned_by: String,
    /// Related statute ID
    pub statute_id: String,
    /// Task status
    pub status: TaskStatus,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Started timestamp
    pub started_at: Option<DateTime<Utc>>,
    /// Completed timestamp
    pub completed_at: Option<DateTime<Utc>>,
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    /// Review notes
    pub notes: Vec<String>,
}

impl ReviewTask {
    /// Creates a new review task.
    pub fn new(
        title: impl Into<String>,
        assigned_to: impl Into<String>,
        assigned_by: impl Into<String>,
        statute_id: impl Into<String>,
    ) -> Self {
        Self {
            task_id: Uuid::new_v4(),
            title: title.into(),
            description: None,
            assigned_to: assigned_to.into(),
            assigned_by: assigned_by.into(),
            statute_id: statute_id.into(),
            status: TaskStatus::NotStarted,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            due_date: None,
            notes: Vec::new(),
        }
    }

    /// Sets description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets due date.
    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Starts the task.
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    /// Completes the task.
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Adds a note.
    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }

    /// Checks if overdue.
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            Utc::now() > due && self.status != TaskStatus::Completed
        } else {
            false
        }
    }
}

/// Task manager.
#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<Uuid, ReviewTask>,
}

impl TaskManager {
    /// Creates a new task manager.
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Creates a task.
    pub fn create_task(&mut self, task: ReviewTask) -> Uuid {
        let id = task.task_id;
        self.tasks.insert(id, task);
        id
    }

    /// Gets a task by ID.
    pub fn get_task(&self, task_id: Uuid) -> Option<&ReviewTask> {
        self.tasks.get(&task_id)
    }

    /// Gets a mutable task by ID.
    pub fn get_task_mut(&mut self, task_id: Uuid) -> Option<&mut ReviewTask> {
        self.tasks.get_mut(&task_id)
    }

    /// Gets tasks assigned to a user.
    pub fn tasks_for_user(&self, user_id: &str) -> Vec<&ReviewTask> {
        self.tasks
            .values()
            .filter(|t| t.assigned_to == user_id)
            .collect()
    }

    /// Gets overdue tasks.
    pub fn overdue_tasks(&self) -> Vec<&ReviewTask> {
        self.tasks.values().filter(|t| t.is_overdue()).collect()
    }

    /// Gets tasks by status.
    pub fn tasks_by_status(&self, status: TaskStatus) -> Vec<&ReviewTask> {
        self.tasks.values().filter(|t| t.status == status).collect()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
