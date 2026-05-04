use crate::http_wrappers::login::verify_credentials;
use crate::sql;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_app::credentials::AccessLevel;
use core_app::requests::Id;
use core_app::types::{Accessories, AuthRequest};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

// Chain Handlers
pub async fn add_accessories(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Accessories>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::add_data::add_accessories(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error adding chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn get_all_accessories(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Accessories>>, StatusCode> {
    let conn = connection.lock().await;
    let _ = verify_credentials(&conn, &auth_request.credentials)?;
    sql::get_data::get_all_accessories(&conn)
        .map(Json)
        .map_err(|e| {
            eprintln!("Error getting all accessories: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn update_accessories(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Accessories>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::set_data::set_accessories(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error updating chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn delete_accessories(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Vec<Id>>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::remove_data::delete_accessories(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error deleting chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}
