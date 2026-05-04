use axum::{
    routing::{get, post}, Json,
    Router,
};

use crate::http_wrappers::accessories::{
    add_accessories, delete_accessories, get_all_accessories, update_accessories,
};
use crate::http_wrappers::chain::{add_chain, delete_chain, get_all_chains, update_chain};
use crate::http_wrappers::login::{login, verify_credentials};
use crate::http_wrappers::section::{
    add_section, delete_section, get_all_sections, update_section,
};
use crate::http_wrappers::user::{add_user, delete_user, get_all_users, update_user};
use crate::sql::create_table::open_db;
use axum::extract::State;
use axum::http::StatusCode;
use core_app::credentials::AccessLevel;
use core_app::replies::Calculation;
use core_app::requests::SelectedBlock;
use core_app::types;
use core_app::types::{Accessories, AuthRequest, Chain, PipelineType, Section, User};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod http_wrappers;
mod sql;

#[tokio::main]
async fn main() {
    let connection = Arc::new(Mutex::new(open_db().await.unwrap()));

    let app = Router::new()
        .route("/login", post(login))
        .route(
            "/sections",
            get(get_all_sections)
                .post(add_section)
                .put(update_section)
                .delete(delete_section),
        )
        .route(
            "/chains",
            get(get_all_chains)
                .post(add_chain)
                .put(update_chain)
                .delete(delete_chain),
        )
        .route(
            "/users",
            get(get_all_users)
                .post(add_user)
                .put(update_user)
                .delete(delete_user),
        )
        .route(
            "/accessories",
            get(get_all_accessories)
                .post(add_accessories)
                .put(update_accessories)
                .delete(delete_accessories),
        )
        .route("/calculations", post(calculate))
        .with_state(connection.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn default(conn: &Connection) {
    sql::add_data::add_user(
        &conn,
        &User {
            id: 0,
            hash: "admin".to_string(),
            name: "admin".to_string(),
            email: "mail".to_string(),
            phone: "89650852981".to_string(),
            level: AccessLevel::Programmer,
        },
    )
    .unwrap();

    sql::add_data::add_user(
        &conn,
        &User {
            id: 1,
            hash: "dev1".to_string(),
            name: "12345678".to_string(),
            email: "mail".to_string(),
            phone: "89112589687".to_string(),
            level: AccessLevel::User,
        },
    )
    .unwrap();
    sql::add_data::add_section(
        &conn,
        &Section {
            id: 0,
            pipeline_type: PipelineType::Lamellar,
            length: 4582,
            price: 456,
            coefficient: 5,
            tags: vec!["First".to_string(), "Second".to_string()],
            opaque: "Additional".to_string(),
            name: "Section 1".to_string(),
        },
    )
    .unwrap();
    sql::add_data::add_chain(
        &conn,
        &Chain {
            id: 2,
            pipeline_type: PipelineType::Lamellar,
            material: types::ChainMaterial::Steel,
            price: 20,
            name: "ARF".to_string(),
            tags: vec!["First".to_string(), "Second".to_string()],
            opaque: "additional".to_string(),
        },
    )
    .unwrap();
    sql::add_data::add_accessories(
        &conn,
        &Accessories {
            id: 10,
            name: "Some chain".to_string(),
            price: 158,
            tags: vec!["First".to_string(), "Second".to_string()],
            opaque: "additional".to_string(),
        },
    )
    .unwrap();
}

// Get sum
// #[debug_handler]
async fn calculate(
    State(connection): State<Arc<Mutex<Connection>>>,
    Json(auth_request): Json<AuthRequest<Vec<SelectedBlock>>>,
) -> Result<Json<Calculation>, StatusCode> {
    let conn = connection.lock().await;
    let _ = verify_credentials(&conn, &auth_request.credentials)?;
    Ok(Json(
        sql::calculate::calculate(&conn, &auth_request.payload).map_err(|e| {
            eprintln!("Error adding section: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
    ))
}
