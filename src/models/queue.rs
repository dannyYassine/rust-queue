use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env::{self, VarError};
use tokio::time::{sleep, Duration};

use super::job::Job;

pub struct Queue {
    connection: Option<PgConnection>,
}

impl Queue {
    pub fn new() -> Self {
        return Queue { connection: None };
    }
    pub fn bootstrap(&mut self) -> &Self {
        let result: Result<String, VarError> = env::var("DATABASE_URL");

        if result.is_err() {
            panic!("Missing DATABASE_URL");
        }

        self.connection = Some(
            PgConnection::establish(&result.unwrap())
                .expect(&format!("Error connecting to {}", &result.unwrap())),
        );

        return &self;
    }
    pub async fn listen(&self) {
        loop {
            let jobs: Vec<Job> =
                diesel::sql_query("SELECT id, payload FROM jobs WHERE status = 'pending'")
                    .load::<Job>(&mut self.connection.unwrap())
                    .expect("Error loading jobs");

            for job in jobs {
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
