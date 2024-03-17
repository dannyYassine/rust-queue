#[derive(Debug, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub payload: String,
    pub status: JobStatus,
}

impl Job {
    pub fn set_status_as_pending(&mut self) -> &Self {
        self.status = JobStatus::Pending;

        return &self;
    }
    pub fn set_status_as_running(&mut self) -> &Self {
        self.status = JobStatus::Running;

        return &self;
    }
    pub fn set_status_as_completed(&mut self) -> &Self {
        self.status = JobStatus::Completed;

        return &self;
    }
}

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
}
