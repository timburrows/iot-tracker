/**
 * Summary: IoT Device Interaction Tracker & Anomaly Detection
 * Author: Tim Burrows
 * Infrastructure: Google Cloud Platform, Postgres 16.x
 */

use std::env;

use entities::{
    device_interactions,
    device,
};

use data::query::{DeviceQuery, DeviceInteractionsQuery};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use dotenvy::dotenv;
use sea_orm::*;

#[derive(Clone)]
struct AppState {
    db_conn: DbConn
}

#[tokio::main]
pub async fn run() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not configured");
    let api_host = env::var("HOST").expect("HOST not configured");
    let api_port = env::var("PORT").expect("PORT not configured");

    println!("Attempting to start Web API service and establish database connectivity...");

    let db = Database::connect(db_url)
        .await
        .expect("Failed to connect to a database. Check that your DATABASE_URL is correct and the DB is started");

    let state = AppState { db_conn: db };

    let app = Router::new()
        .route("/", get(|| async { "Alive" }))
        .route("/device-interaction", post(create_dev_interaction))
        .route("/device-interaction/:id", get(list_device_interactions))
        .route("/device", get(list_devices))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", api_host, api_port))
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_dev_interaction(
    _state: State<AppState>,
) -> impl IntoResponse {
    panic!("Not implemented");
    // Response::builder()
    //     .status(StatusCode::CREATED)
    //     .body(Body::from("Device Interaction record created successfully"))
    //     .unwrap()
}

async fn list_device_interactions(
    state: State<AppState>,
    Path(id): Path<i32>
) -> Json<Vec<device_interactions::Model>> {
    let data = DeviceInteractionsQuery::list_device_interactions(&state.db_conn, id).await.expect("No device interactions");
    Json(data)
}

async fn list_devices(
    state: State<AppState>,
) -> Json<Vec<device::Model>> {
    let data = DeviceQuery::list_devices(&state.db_conn).await.expect("Could not find any devices");
    Json(data)
}