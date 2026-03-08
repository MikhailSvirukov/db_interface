use core_app::credentials::{AccessLevel, Credentials};
use core_app::requests::Id;
use core_app::types::{Accessories, Chain, ChainMaterial, PipelineType, Section, User};
use num::FromPrimitive;
use rusqlite::{params, Connection, Result};
use serde_json;

pub fn get_all_sections(connection: &Connection) -> Result<Vec<Section>> {
    let mut stmt = connection.prepare(
        "SELECT id, pipeline_type, length, price, tags, coef, opaque, name FROM sections",
    )?;
    let sections_iter = stmt.query_map(params![], |row| {
        let tags_json: String = row.get(4)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Section {
            id: row.get(0)?,
            name: row.get(7)?,
            pipeline_type: PipelineType::from_i32(row.get(1)?).unwrap(),
            length: row.get(2)?,
            price: row.get(3)?,
            tags,
            coefficient: row.get(5)?,
            opaque: row.get(6)?,
        })
    })?;
    sections_iter.collect()
}

pub fn get_section_by_id(connection: &Connection, id: Id) -> Result<Section> {
    let mut stmt = connection.prepare(
        "SELECT id, pipeline_type, length, price, tags, coef, opaque, name
         FROM sections
         WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        let tags_json: String = row.get(4)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Section {
            id: row.get(0)?,
            name: row.get(7)?,
            pipeline_type: PipelineType::from_i32(row.get(1)?).unwrap(),
            length: row.get(2)?,
            price: row.get(3)?,
            tags,
            coefficient: row.get(5)?,
            opaque: row.get(6)?,
        })
    })
}

pub fn get_all_chains(connection: &Connection) -> Result<Vec<Chain>> {
    let mut stmt = connection
        .prepare("SELECT pipeline_type, material, price, name, id, tags, opaque FROM chains")?;
    let chains_iter = stmt.query_map(params![], |row| {
        let tags_json: String = row.get(5)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Chain {
            id: row.get(4)?,
            pipeline_type: PipelineType::from_i32(row.get(0)?).unwrap(),
            material: ChainMaterial::from_i32(row.get(1)?).unwrap(),
            price: row.get(2)?,
            name: row.get(3)?,
            tags,
            opaque: row.get(6)?,
        })
    })?;
    chains_iter.collect()
}

pub fn get_chain_by_id(connection: &Connection, id: Id) -> Result<Chain> {
    let mut stmt = connection.prepare(
        "SELECT pipeline_type, material, price, name, id, tags, opaque FROM chains WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        let tags_json: String = row.get(5)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Chain {
            id: row.get(4)?,
            pipeline_type: PipelineType::from_i32(row.get(0)?).unwrap(),
            material: ChainMaterial::from_i32(row.get(1)?).unwrap(),
            price: row.get(2)?,
            name: row.get(3)?,
            tags,
            opaque: row.get(6)?,
        })
    })
}

pub fn get_all_users(connection: &Connection) -> Result<Vec<User>> {
    let mut stmt = connection.prepare("SELECT hash, email, name, phone, level, id FROM users")?;
    let users_iter = stmt.query_map(params![], |row| {
        Ok(User {
            id: row.get(5)?,
            hash: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            phone: row.get(3)?,
            level: AccessLevel::from_i32(row.get(4)?).unwrap(),
        })
    })?;
    users_iter.collect()
}

pub fn get_all_accessories(connection: &Connection) -> Result<Vec<Accessories>> {
    let mut stmt = connection.prepare("SELECT id, name, price, tags, opaque FROM accessories")?;
    let accessories_iter = stmt.query_map(params![], |row| {
        let tags_json: String = row.get(3)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Accessories {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            tags,
            opaque: row.get(4)?,
        })
    })?;
    accessories_iter.collect()
}

pub fn get_accessories_by_id(connection: &Connection, id: Id) -> Result<Accessories> {
    let mut stmt = connection
        .prepare("SELECT id, name, price, tags, opaque FROM accessories WHERE id = ?1")?;
    stmt.query_row([id], |row| {
        let tags_json: String = row.get(3)?;
        let tags: Vec<String> =
            serde_json::from_str(&tags_json).expect("Failed to deserialize tags");
        Ok(Accessories {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            tags,
            opaque: row.get(3)?,
        })
    })
}

pub fn get_user_name(connection: &Connection, name: String) -> Result<Credentials> {
    let mut stmt = connection.prepare("SELECT name, hash, level FROM users WHERE name = ?1")?;
    stmt.query_row([name], |row| {
        Ok(Credentials {
            login: row.get(0)?,
            password: row.get(1)?,
            access_level: AccessLevel::from_i32(row.get(2)?).unwrap(),
        })
    })
}
