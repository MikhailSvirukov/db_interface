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
    let connection = Arc::new(Mutex::new(open_db().unwrap()));
    default(connection.clone()).await;

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
        .route("/accessories/add", post(add_accessories))
        .route("/accessories/get", get(get_all_accessories))
        .route("/accessories/update", post(update_accessories))
        .route("/accessories/delete", post(delete_accessories))
        .route("/calculation", post(calculate))
        .with_state(connection.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn default(connection: Arc<Mutex<Connection>>) {
    // Добавление пользователей

    let conn = connection.lock().await;
    sql::add_data::add_user(
        &conn,
        &User {
            id: 0,
            hash: "12345678".to_string(),
            name: "12345678".to_string(),
            email: "mail".to_string(),
            phone: "89652".to_string(),
            level: AccessLevel::Programmer,
        },
    )
    .unwrap();

    sql::add_data::add_user(
        &conn,
        &User {
            id: 1,
            hash: "1234".to_string(),
            name: "1234".to_string(),
            email: "mail".to_string(),
            phone: "89652".to_string(),
            level: AccessLevel::User,
        },
    )
    .unwrap();
    sql::add_data::add_section(
        &conn,
        &Section {
            id: 0,
            pipeline_type: PipelineType::Lamellar,
            section_type: types::Type::Driving,
            length: 4582,
            price: 456,
            is_magnet: true,
            material_sides: types::SideMaterial::Steel,
            radius: 0,
            angle: 0,
            tags: vec!["First".to_string(), "Second".to_string()],
        },
    )
    .unwrap();
    sql::add_data::add_chain(
        &conn,
        &Chain {
            id: 2,
            pipeline_type: PipelineType::Lamellar,
            chain_type: types::Type::Driving,
            material: types::ChainMaterial::Steel,
            price: 20,
            is_magnet: true,
            name: "ARF".to_string(),
            tags: vec!["First".to_string(), "Second".to_string()],
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
