use core_app::types::{Accessories, Chain, Section, User};
use num::ToPrimitive;
use rusqlite::{params, Connection};
use serde_json;

// let's assume for now, that user inserts all fields to update
pub fn set_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&section.tags).expect("Failed to serialize chains");
    connection.execute(
        "UPDATE sections SET pipeline_type = ?1, length = ?2, price = ?3, tags = ?4, coef =?5, opaque =?6, name =?7
            WHERE id = ?8",
        (
            section.pipeline_type.to_i32().unwrap(),
            section.length as i32,
            section.price as i32,
            tags_json,
            section.coefficient,
            section.opaque.clone(),
            section.name.clone(),
            section.id,
        ),
    )
}

pub fn set_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&chain.tags).expect("Failed to serialize chains");
    connection.execute(
        "UPDATE chains SET pipeline_type = ?1, material = ?2, price = ?3, name = ?4, tags = ?5, opaque = ?6 WHERE id = ?7",
        (chain.pipeline_type.to_i32().unwrap(),
         chain.material.to_i32().unwrap(),
         chain.price as i32,
         &chain.name,
            tags_json,
            chain.opaque.clone(),
         chain.id
        )
    )
}

pub fn set_user(connection: &Connection, user: &User) -> rusqlite::Result<usize> {
    connection.execute(
        "UPDATE users SET hash = ?1, email = ?2, name = ?3, phone = ?4, level = ?5 WHERE id = ?6",
        (
            &user.hash,
            &user.email,
            &user.name,
            user.phone.parse::<i32>().unwrap(),
            user.level.to_i32().unwrap(),
            user.id,
        ),
    )
}

pub fn set_accessories(
    connection: &Connection,
    accessories: &Accessories,
) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&accessories.tags).expect("Failed to serialize chains");
    connection.execute(
        "UPDATE accessories SET name = ?1, price = ?2, tags = ?3, opaque =?4 WHERE id = ?5",
        params![
            accessories.name,
            accessories.price,
            tags_json,
            accessories.opaque.clone(),
            accessories.id
        ],
    )
}
