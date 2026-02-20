use crate::credentials::{AccessLevel, Credentials};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive, Clone)]
pub enum Type {
    Driving = 0,
    Finite = 1,
    Intermediate = 2,
    Turning = 3,
    DoubleRow = 4,
    TripleRow12 = 5,
    TripleRow21 = 6,
}

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Driving" => Ok(Type::Driving),
            "Finite" => Ok(Type::Finite),
            "Intermediate" => Ok(Type::Intermediate),
            "Turning" => Ok(Type::Turning),
            "DoubleRow" => Ok(Type::DoubleRow),
            "TripleRow12" => Ok(Type::TripleRow12),
            "TripleRow21" => Ok(Type::TripleRow21),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToPrimitive, FromPrimitive, Clone)]
pub enum SideMaterial {
    Steel = 0,
}

impl FromStr for SideMaterial {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Steel" => Ok(SideMaterial::Steel),
            _ => Err(format!("Unknown side material: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Section {
    pub id: isize,
    pub section_type: Type,
    pub width: isize,
    pub length: isize,
    pub price: isize,
    pub is_magnet: bool,
    pub material_sides: SideMaterial,
    pub radius: isize,
    pub angle: isize,
    pub chains: Vec<Chain>,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive, Clone)]
pub enum ChainMaterial {
    Steel = 0,
    Plastic = 1,
}

impl FromStr for ChainMaterial {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Steel" => Ok(ChainMaterial::Steel),
            "Plastic" => Ok(ChainMaterial::Plastic),
            _ => Err(format!("Unknown chain material: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub id: isize,
    pub chain_type: Type,
    pub material: ChainMaterial,
    pub width: isize,
    pub price: isize,
    pub is_magnet: bool,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: isize,
    pub hash: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub level: AccessLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest<T> {
    pub credentials: Credentials,
    pub payload: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthReply<T> {
    pub credentials: Credentials,
    pub payload: T,
}
