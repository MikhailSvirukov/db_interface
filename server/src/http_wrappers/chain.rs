use crate::http_wrappers::login::verify_credentials;
use crate::sql;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_app::credentials::AccessLevel;
use core_app::requests::Id;
use core_app::types::{AuthRequest, Chain};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

// Chain Handlers
pub async fn add_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::add_data::add_chain(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error adding chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn get_all_chains(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Chain>>, StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::User
        | AccessLevel::Economist
        | AccessLevel::Manager
        | AccessLevel::Administrator
        | AccessLevel::Programmer => sql::get_data::get_all_chains(&conn).map(Json).map_err(|e| {
            eprintln!("Error getting all chains: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn update_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::set_data::set_chain(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error updating chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn delete_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Vec<Id>>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials)?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::remove_data::delete_chain(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error deleting chain: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}
