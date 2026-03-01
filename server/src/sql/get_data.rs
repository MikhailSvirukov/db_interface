use core_app::credentials::{AccessLevel, Credentials};
use core_app::requests::Id;
use core_app::types::{
    Accessories, Chain, ChainMaterial, PipelineType, Section, SideMaterial, Type, User,
};
use num::FromPrimitive;
use rusqlite::{params, Connection, Result};
use serde_json;

pub fn get_all_sections(connection: &Connection) -> Result<Vec<Section>> {
    let mut stmt = connection.prepare("SELECT id, type, length, price, is_magnet, material_sides, radius, angle, chains, pipeline_type FROM sections")?;
    let sections_iter = stmt.query_map(params![], |row| {
        let chains_json: String = row.get(8)?;
        let chains: Vec<Id> =
            serde_json::from_str(&chains_json).expect("Failed to deserialize chains");
        Ok(Section {
            id: row.get(0)?,
            pipeline_type: PipelineType::from_i32(row.get(9)?).unwrap(),
            section_type: Type::from_i32(row.get(1)?).unwrap(),
            length: row.get(2)?,
            price: row.get(3)?,
            is_magnet: row.get(4)?,
            material_sides: SideMaterial::from_i32(row.get(5)?).unwrap(),
            radius: row.get(6)?,
            angle: row.get(7)?,
            chains,
        })
    })?;
    sections_iter.collect()
}

pub fn get_section_by_id(connection: &Connection, id: Id) -> Result<Section> {
    let mut stmt = connection.prepare(
        "SELECT id, type, length, price, is_magnet, material_sides, radius, angle, chains, pipeline_type
         FROM sections
         WHERE id = ?1"
    )?;

    stmt.query_row([id], |row| {
        let chains_json: String = row.get(8)?;
        let chains: Vec<Id> =
            serde_json::from_str(&chains_json).expect("Failed to deserialize chains");

        Ok(Section {
            id: row.get(0)?,
            pipeline_type: PipelineType::from_i32(row.get(9)?).unwrap(),
            section_type: Type::from_i32(row.get(1)?).unwrap(),
            length: row.get(2)?,
            price: row.get(3)?,
            is_magnet: row.get(4)?,
            material_sides: SideMaterial::from_i32(row.get(5)?).unwrap(),
            radius: row.get(6)?,
            angle: row.get(7)?,
            chains,
        })
    })
}

pub fn get_all_chains(connection: &Connection) -> Result<Vec<Chain>> {
    let mut stmt = connection.prepare(
        "SELECT chain_type, material, price, is_magnet, name, id, pipeline_type FROM chains",
    )?;
    let chains_iter = stmt.query_map(params![], |row| {
        Ok(Chain {
            id: row.get(5)?,
            pipeline_type: PipelineType::from_i32(row.get(6)?).unwrap(),
            chain_type: Type::from_i32(row.get(0)?).unwrap(),
            material: ChainMaterial::from_i32(row.get(1)?).unwrap(),
            price: row.get(2)?,
            is_magnet: row.get(3)?,
            name: row.get(4)?,
        })
    })?;
    chains_iter.collect()
}

pub fn get_chain_by_id(connection: &Connection, id: Id) -> Result<Chain> {
    let mut stmt = connection.prepare(
        "SELECT chain_type, material, price, is_magnet, name, id, pipeline_type FROM chains WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        Ok(Chain {
            id: row.get(5)?,
            pipeline_type: PipelineType::from_i32(row.get(6)?).unwrap(),
            chain_type: Type::from_i32(row.get(0)?).unwrap(),
            material: ChainMaterial::from_i32(row.get(1)?).unwrap(),
            price: row.get(2)?,
            is_magnet: row.get(3)?,
            name: row.get(4)?,
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
    let mut stmt = connection.prepare("SELECT id, name FROM accessories")?;
    let accessories_iter = stmt.query_map(params![], |row| {
        Ok(Accessories {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    accessories_iter.collect()
}

pub fn get_accessories_by_id(connection: &Connection, id: Id) -> Result<Accessories> {
    let mut stmt = connection.prepare("SELECT id, name FROM accessories WHERE id = ?1")?;
    stmt.query_row([id], |row| {
        Ok(Accessories {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })
}

pub fn get_user_name(connection: &Connection, name: String) -> Result<Credentials> {
    let mut stmt = connection.prepare("SELECT name, hash, level FROM users WHERE name = ?1")?;
    let rows = stmt.query_map([name], |row| {
        Ok(Credentials {
            login: row.get(0)?,
            password: row.get(1)?,
            access_level: AccessLevel::from_i32(row.get(2)?).unwrap(),
        })
    })?;
    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }
    if names.len() == 1 {
        Ok(names.remove(0))
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}
