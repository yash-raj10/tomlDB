use axum::{body::Body, routing::{get, post}, Router};
// Adjust the import path to match the actual location of the `hello_world` function
use crate::fns::{hello_world, data_store};

pub fn routess() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/data_store", get(data_store))
}

