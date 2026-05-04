use core_app::types::{Accessories, Chain, Section, User};
use num::ToPrimitive;
use rusqlite::Connection;
use serde_json;

pub fn add_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&section.tags).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO sections (pipeline_type, length, price, tags, coef, opaque, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            section.pipeline_type.to_i32().unwrap(),
            section.length as i32,
            section.price as i32,
            tags_json,
            section.coefficient as i32,
            section.opaque.clone(),
            section.name.clone(),
        ),
    )
}

pub fn add_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&chain.tags).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO chains (pipeline_type, material, price, name, tags, opaque) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (chain.pipeline_type.to_i32().unwrap(),
         chain.material.to_i32().unwrap(),
         chain.price as i32,
         &chain.name,
            tags_json,
            chain.opaque.clone(),
        )
    )
}

pub fn add_user(connection: &Connection, user: &User) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO users (hash, email, name, phone, level) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &user.hash,
            &user.email,
            &user.name,
            &user.phone,
            user.level.to_i32().unwrap(),
        ),
    )
}

pub fn add_accessories(
    connection: &Connection,
    accessories: &Accessories,
) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&accessories.tags).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO accessories (name, price, tags, opaque) VALUES (?1, ?2, ?3, ?4)",
        (
            &accessories.name,
            accessories.price,
            &tags_json,
            &accessories.opaque,
        ),
    )
}
