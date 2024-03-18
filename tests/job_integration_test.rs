#[cfg(test)]
mod tests {
    use rust_queue::models::job::{Job, JobStatus};

    #[test]
    fn it_should_set_job_status_to_pending() {
        let mut job: Job = Job {
            id: 1,
            payload: "".to_string(),
            status: "".to_string(),
        };

        job.set_status_as_pending();

        assert_eq!(job.status, JobStatus::Pending.to_string());
    }

    #[test]
    fn it_should_set_job_status_to_running() {
        let mut job: Job = Job {
            id: 1,
            payload: "".to_string(),
            status: "".to_string(),
        };

        job.set_status_as_running();

        assert_eq!(job.status, JobStatus::Running.to_string());
    }

    #[test]
    fn it_should_set_job_status_to_completed() {
        let mut job: Job = Job {
            id: 1,
            payload: "".to_string(),
            status: "".to_string(),
        };

        job.set_status_as_completed();

        assert_eq!(job.status, JobStatus::Completed.to_string());
    }
}
