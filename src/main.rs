use std::env;

/**
 * Summary: IoT Device Interaction Tracker & Anomaly Detection
 * Author: Tim Burrows
 * Infrastructure: Google Cloud Platform, Postgres 16.x
 */


use dotenvy::dotenv;
use sea_orm::*;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not configured");


    println!("Attempting to start Web API service and establish database connectivity...");

    let _db = Database::connect(db_url).await
        .expect("Failed to connect to a database. Check that your DATABASE_URL is correct and the DB is started");
}
