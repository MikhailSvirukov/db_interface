use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use core_app::types::{AuthRequest, Chain, Section, User};
use core_app::credentials::{Credentials, AccessLevel};
use rusqlite::Connection;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use crate::sql::create_table::open_db;

mod calculations;
mod sql;

#[tokio::main]
async fn main() {
    let connection = Arc::new(Mutex::new(open_db().unwrap()));
    let app = Router::new()
        .route("/section", post(add_section))
        .route("/sections", get(get_all_sections))
        .route("/chain", post(add_chain))
        .route("/chains", get(get_all_chains))
        .route("/user", post(add_user))
        .route("/users", get(get_all_users))
        .with_state(connection);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Helper function to verify credentials and determine access level
async fn verify_credentials(
    connection: &Connection,
    credentials: &Credentials,
) -> Result<AccessLevel, (StatusCode, String)> {
    // In a real application, you would hash the provided password and compare it
    // with the stored hash in the database for the given login.
    // For this example, we'll use a very simple (and insecure) check.
    match credentials.login.as_str() {
        "admin" if credentials.password == "adminpass" => Ok(AccessLevel::Administrator),
        "user" if credentials.password == "userpass" => Ok(AccessLevel::User),
        _ => {
            eprintln!("Unauthorized attempt for login: {}", credentials.login);
            Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
        }
    }
}

// Section Handlers
async fn add_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Section>>,
) -> Result<Json<()>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            println!("adding section");
            sql::add_data::add_section(&conn, &auth_request.payload)
                .map(|_| Json(()))
                .map_err(|e| {
                    eprintln!("Error adding section: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}

async fn get_all_sections(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Section>>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User | AccessLevel::Economist | AccessLevel::Manager | AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::get_data::get_all_sections(&conn)
                .map(Json)
                .map_err(|e| {
                    eprintln!("Error getting all sections: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}

// Chain Handlers
async fn add_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<Json<()>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::add_data::add_chain(&conn, &auth_request.payload)
                .map(|_| Json(()))
                .map_err(|e| {
                    eprintln!("Error adding chain: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}

async fn get_all_chains(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Chain>>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User | AccessLevel::Economist | AccessLevel::Manager | AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::get_data::get_all_chains(&conn)
                .map(Json)
                .map_err(|e| {
                    eprintln!("Error getting all chains: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}

// User Handlers
async fn add_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<Json<()>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::add_data::add_user(&conn, &auth_request.payload)
                .map(|_| Json(()))
                .map_err(|e| {
                    eprintln!("Error adding user: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}

async fn get_all_users(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = connection.lock().await;
    let access_level = verify_credentials(&conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User | AccessLevel::Economist | AccessLevel::Manager | AccessLevel::Administrator | AccessLevel::Programmer => {
            sql::get_data::get_all_users(&conn)
                .map(Json)
                .map_err(|e| {
                    eprintln!("Error getting all users: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })
        }
        _ => Err((StatusCode::FORBIDDEN, "Insufficient access rights".to_string())),
    }
}
