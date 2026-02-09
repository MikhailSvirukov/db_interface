use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use crate::sql::create_table::open_db;
use core_app::credentials::{AccessLevel, Credentials};
use core_app::types::{AuthRequest, Chain, Section, User};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use crate::sql::get_data::get_user_name;
use std::collections::HashMap;
use core_app::types::AuthReply;

mod calculations;
mod sql;

#[tokio::main]
async fn main() {
    let connection = Arc::new(Mutex::new(open_db().unwrap()));
    let app = Router::new()
        .route("/login", post(login))
        .route("/section/add", post(add_section))
        .route("/section/get", get(get_all_sections))
        .route("/section/update", post(update_section))
        .route("/section/delete", post(delete_section))
        .route("/chain/add", post(add_chain))
        .route("/chain/get", get(get_all_chains))
        .route("/chain/update", post(update_chain))
        .route("/chain/delete", post(delete_chain))
        .route("/user/add", post(add_user))
        .route("/user/get", get(get_all_users))
        .route("/user/update", post(update_user))
        .route("/user/delete", post(delete_user))
        .with_state(connection);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Helper function to verify credentials and determine access level
async fn verify_credentials<'a>(
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

async fn login(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<AuthReply<HashMap<String, serde_json::Value>>>, StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User
        | AccessLevel::Economist
        | AccessLevel::Manager
        | AccessLevel::Administrator
        | AccessLevel::Programmer => {
            let mut tables_data: HashMap<String, serde_json::Value> = HashMap::new();

            let sections = sql::get_data::get_all_sections(&conn)
                .map_err(|e| {
                    eprintln!("Error getting sections: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            tables_data.insert("sections".to_string(), serde_json::to_value(sections).unwrap());

            let chains = sql::get_data::get_all_chains(&conn)
                .map_err(|e| {
                    eprintln!("Error getting chains: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            tables_data.insert("chains".to_string(), serde_json::to_value(chains).unwrap());
            
            if access_level == AccessLevel::Administrator || access_level == AccessLevel::Programmer {
                let users = sql::get_data::get_all_users(&conn)
                    .map_err(|e| {
                        eprintln!("Error getting users: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
                tables_data.insert("users".to_string(), serde_json::to_value(users).unwrap());
            }

            Ok(Json(AuthReply {
                credentials: auth_request.credentials,
                payload: tables_data,
            }))
        }
    }
}

// Section Handlers
async fn add_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Section>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator | AccessLevel::Programmer => {
            println!("adding section");
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

async fn get_all_sections(
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

async fn update_section(
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

async fn delete_section(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Section>>,
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

// Chain Handlers
async fn add_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

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

async fn get_all_chains(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<()>>,
) -> Result<Json<Vec<Chain>>, StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::User
        | AccessLevel::Economist
        | AccessLevel::Manager
        | AccessLevel::Administrator
        | AccessLevel::Programmer => sql::get_data::get_all_chains(&conn).map(Json).map_err(|e| {
            eprintln!("Error getting all chains: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }),
    }
}

async fn update_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

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

async fn delete_chain(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Chain>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

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

// User Handlers
async fn add_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator => sql::add_data::add_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error adding user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

async fn get_all_users(
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
    }
}

async fn update_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator => sql::set_data::set_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error updating user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

async fn delete_user(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<User>>,
) -> Result<(), StatusCode> {
    let conn = connection.lock().await;
    let (access_level, conn) = verify_credentials(conn, &auth_request.credentials).await?;

    match access_level {
        AccessLevel::Administrator => sql::remove_data::delete_user(&conn, &auth_request.payload)
            .map(|_| ())
            .map_err(|e| {
                eprintln!("Error deleting user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        _ => Err(StatusCode::FORBIDDEN),
    }
}
