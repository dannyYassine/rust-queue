// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> i32,
        payload -> String,
    }
}