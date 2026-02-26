use crate::http_wrappers::login::verify_credentials;
use crate::sql;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_app::credentials::AccessLevel;
use core_app::requests::Id;
use core_app::types::{AuthRequest, User};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

// User Handlers
pub async fn add_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Programmer => sql::add_data::add_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error adding user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn get_all_users(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User
        | AccessLevel::Economist
        | AccessLevel::Manager
        | AccessLevel::Administrator
        | AccessLevel::Programmer => sql::get_data::get_all_users(&conn).map(Json).map_err(|e| {
            eprintln!("Error getting all users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn update_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Programmer => sql::set_data::set_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error updating user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn delete_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Vec<Id>>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Programmer => sql::remove_data::delete_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error deleting user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}
