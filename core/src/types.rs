use crate::credentials::{AccessLevel, Credentials};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive, Clone, PartialEq)]
pub enum PipelineType {
    None = 0,
    Madal = 1,
    Rolgang = 2,
    Lamellar = 3,
}

impl Display for PipelineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineType::Lamellar => write!(f, "Пластинчатая цепь"),
            PipelineType::Madal => write!(f, "Лента"),
            PipelineType::Rolgang => write!(f, "Ролики"),
            PipelineType::None => write!(f, ""),
        }
    }
}

impl FromStr for PipelineType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Пластинчатая цепь" => Ok(PipelineType::Lamellar),
            "Лента" => Ok(PipelineType::Madal),
            "Ролики" => Ok(PipelineType::Rolgang),
            _ => Err(format!("Invalid pipeline type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToPrimitive, FromPrimitive, Clone)]
pub enum SideMaterial {
    Steel = 0,
}
impl Display for SideMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SideMaterial::Steel => "Сталь".to_string(),
            }
        )
    }
}

impl FromStr for SideMaterial {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Сталь" => Ok(SideMaterial::Steel),
            _ => Err(format!("Unknown side material: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Section {
    pub id: isize,
    pub pipeline_type: PipelineType,
    pub length: isize,
    pub price: isize,
    pub coefficient: isize,
    pub tags: Vec<String>,
    pub opaque: String,
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
            "Сталь" => Ok(ChainMaterial::Steel),
            "Пластик" => Ok(ChainMaterial::Plastic),
            _ => Err(format!("Unknown chain material: {}", s)),
        }
    }
}

impl Display for ChainMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChainMaterial::Steel => "Сталь".to_string(),
            ChainMaterial::Plastic => "Пластик".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub id: isize,
    pub pipeline_type: PipelineType,
    pub material: ChainMaterial,
    pub price: isize,
    pub name: String,
    pub tags: Vec<String>,
    pub opaque: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: isize,
    pub hash: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub level: AccessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accessories {
    pub id: isize,
    pub name: String,
    pub price: isize,
    pub tags: Vec<String>,
    pub opaque: String,
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
