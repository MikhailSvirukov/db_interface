use core_app::types::{Accessories, Chain, Section, User};
use num::ToPrimitive;
use rusqlite::{params, Connection};
use serde_json;

// let's assume for now, that user inserts all fields to update
pub fn set_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    let chains_json = serde_json::to_string(&section.chains).expect("Failed to serialize chains");
    connection.execute(
        "UPDATE sections SET type = ?1, length = ?2, price = ?3, is_magnet = ?4,
            material_sides = ?5, radius = ?6, angle = ?7, chains = ?8, pipeline_type = ?9
            WHERE id = ?10",
        (
            section.section_type.to_i32().unwrap(),
            section.length as i32,
            section.price as i32,
            section.is_magnet,
            section.material_sides.to_i32().unwrap(),
            section.radius as i32,
            section.angle as i32,
            chains_json,
            section.pipeline_type.to_i32().unwrap(),
            section.id,
        ),
    )
}

pub fn set_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    connection.execute(
        "UPDATE chains SET chain_type = ?1, material = ?2, price = ?3, is_magnet =?4, name = ?5, pipeline_type = ?6 WHERE id = ?7",
        (chain.chain_type.to_i32().unwrap(),
         chain.material.to_i32().unwrap(),
         chain.price as i32,
         chain.is_magnet,
         &chain.name,
            chain.pipeline_type.to_i32().unwrap(),
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
    connection.execute(
        "UPDATE accessories SET name = ?1, price = ?2 WHERE id = ?3",
        params![accessories.name, accessories.price, accessories.id],
    )
}
