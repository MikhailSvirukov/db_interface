use core_app::types::{Chain, Section, User};
use num::ToPrimitive;
use rusqlite::Connection;
use serde_json;

pub fn add_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    let chains_json = serde_json::to_string(&section.chains).expect("Failed to serialize chains");
    connection.execute(
        "INSERT INTO sections (type, width, length, price, is_magnet, material_sides, radius, angle, chains) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (section.section_type.to_i32().unwrap(),
            section.width as i32,
            section.length as i32,
            section.price as i32,
            section.is_magnet,
            section.material_sides.to_i32().unwrap(),
            section.radius as i32,
            section.angle as i32,
            chains_json,
        )
    )
}

pub fn add_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO chains (chain_type, material, width, price, is_magnet, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (chain.chain_type.to_i32().unwrap(),
         chain.material.to_i32().unwrap(),
         chain.width as i32,
         chain.price as i32,
         chain.is_magnet,
         &chain.name,
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
            user.phone.parse::<i32>().unwrap(),
            user.level.to_i32().unwrap(),
        ),
    )
}
