use crate::sql::get_data::get_user_name;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_app::credentials::{AccessLevel, Credentials};
use core_app::types::{AuthReply, AuthRequest};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

// Helper function to verify credentials and determine access level
pub async fn verify_credentials<'a>(
    connection: MutexGuard<'a, Connection>,
    credentials: &Credentials,
) -> Result<(AccessLevel, MutexGuard<'a, Connection>), StatusCode> {
    let hash = get_user_name(&connection, credentials.login.clone());
    match hash {
        Ok(cred) => {
            if cred.password == credentials.password {
                Ok((cred.access_level, connection))
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(err) => {
            eprintln!("Error getting credentials: {}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn login(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<AuthReply<()>>, StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    Ok(Json(AuthReply {
        credentials: Credentials {
            login: auth_request.credentials.login,
            password: auth_request.credentials.password,
            access_level,
        },
        payload: (),
    }))
}
