pub mod models;
pub mod schema;
pub mod routes;
pub mod jwt;

#[macro_use]
extern crate diesel;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database};
///
/// database connection pool
///
#[database("db")]
pub struct PgConnection(diesel::PgConnection);

///
/// error type
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}

impl ApiError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}