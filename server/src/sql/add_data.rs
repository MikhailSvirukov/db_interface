use core_app::types::{Accessories, Chain, Section, User};
use num::ToPrimitive;
use rusqlite::{params, Connection};
use serde_json;

pub fn add_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&section.tags).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO sections (type, length, price, is_magnet, material_sides, radius, angle, tags, pipeline_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (section.section_type.to_i32().unwrap(),
            section.length as i32,
            section.price as i32,
            section.is_magnet,
            section.material_sides.to_i32().unwrap(),
            section.radius as i32,
            section.angle as i32,
            tags_json,
            section.pipeline_type.to_i32().unwrap(),
        )
    )
}

pub fn add_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    let tags_json = serde_json::to_string(&chain.tags).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO chains (chain_type, material, price, is_magnet, name, pipeline_type, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (chain.chain_type.to_i32().unwrap(),
         chain.material.to_i32().unwrap(),
         chain.price as i32,
         chain.is_magnet,
         &chain.name,
         chain.pipeline_type.to_i32().unwrap(),
            tags_json,
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
        "INSERT INTO accessories (name, price, tags) VALUES (?1, ?2, ?3)",
        params![&accessories.name, accessories.price, tags_json],
    )
}
