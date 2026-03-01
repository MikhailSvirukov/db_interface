use crate::credentials::{AccessLevel, Credentials};
use crate::requests::Id;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive, Clone, PartialEq)]
pub enum PipelineType {
    Lamellar = 3,
    Madal = 1,
    Rolgang = 2,
    None = 0,
}

impl Display for PipelineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineType::Lamellar => write!(f, "Пластинчатый"),
            PipelineType::Madal => write!(f, "Модальный"),
            PipelineType::Rolgang => write!(f, "Рольганг"),
            PipelineType::None => write!(f, ""),
        }
    }
}

impl FromStr for PipelineType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Пластинчатый" => Ok(PipelineType::Lamellar),
            "Модальный" => Ok(PipelineType::Madal),
            "Рольганг" => Ok(PipelineType::Rolgang),
            _ => Err(format!("Invalid pipeline type: {}", s)),
        }
    }
}

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
            "Приводящая" => Ok(Type::Driving),
            "Конечная" => Ok(Type::Finite),
            "Промежуточная" => Ok(Type::Intermediate),
            "Поворотная" => Ok(Type::Turning),
            "Двойная" => Ok(Type::DoubleRow),
            "Тройная 1к2" => Ok(Type::TripleRow12),
            "Тройная 2к1" => Ok(Type::TripleRow21),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Type::Driving => "Приводящая".to_string(),
            Type::Finite => "Конечная".to_string(),
            Type::Intermediate => "Промежуточная".to_string(),
            Type::Turning => "Поворотная".to_string(),
            Type::DoubleRow => "Двойная".to_string(),
            Type::TripleRow12 => "Тройная 1к2".to_string(),
            Type::TripleRow21 => "Тройная 2к1".to_string(),
        };
        write!(f, "{}", str)
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
    pub section_type: Type,
    pub length: isize,
    pub price: isize,
    pub is_magnet: bool,
    pub material_sides: SideMaterial,
    pub radius: isize,
    pub angle: isize,
    pub chains: Vec<Id>,
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
    pub chain_type: Type,
    pub material: ChainMaterial,
    pub price: isize,
    pub is_magnet: bool,
    pub name: String,
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
