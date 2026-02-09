use serde::{Serialize, Deserialize};
use num_derive::{FromPrimitive, ToPrimitive};
use crate::credentials::{AccessLevel, Credentials};

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum Type {
    Driving = 0,
    Finite = 1,
    Intermediate = 2,
    Turning = 3,
    DoubleRow = 4,
    TripleRow12 = 5,
    TripleRow21 = 6,
}

#[derive(Debug, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SideMaterial {
    Steel = 0,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum ChainMaterial {
    Steel = 0,
    Plastic = 1,
}

#[derive(Debug, Serialize, Deserialize)]
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