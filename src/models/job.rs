use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub payload: String,
}
