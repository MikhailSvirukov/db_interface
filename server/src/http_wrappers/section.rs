use crate::http_wrappers::login::verify_credentials;
use crate::sql;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_app::credentials::AccessLevel;
use core_app::requests::Id;
use core_app::types::{AuthRequest, Section};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

// Section Handlers
pub async fn add_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Section>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::add_data::add_section(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error adding section: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn get_all_sections(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Section>>, StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User
        | AccessLevel::Economist
        | AccessLevel::Manager
        | AccessLevel::Administrator
        | AccessLevel::Programmer => {
            sql::get_data::get_all_sections(&conn)
                .map(Json)
                .map_err(|e| {
                    eprintln!("Error getting all sections: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
    }
}

pub async fn update_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Section>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::set_data::set_section(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error updating section: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}

pub async fn delete_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Vec<Id>>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::remove_data::delete_section(&conn, &auth_request.payload)
                .map(|_| ())
                .map_err(|e| {
                    eprintln!("Error deleting section: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        _ => Err(StatusCode::FORBIDDEN),
    }
}
