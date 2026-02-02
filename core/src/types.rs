use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Driving,
    Finite,
    Intermediate,
    Turning,
    DoubleRow,
    TripleRow12,
    TripleRow21,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SideMaterial {
    Steel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: usize,
    pub section_type: Type,
    pub width: usize,
    pub length: usize,
    pub price: usize,
    pub is_magnit: bool,
    pub material_sides: SideMaterial,
    pub radius: usize,
    pub angle: usize,
    pub chains: Vec<Chain>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChainMaterial {
    Steel,
    Plastic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub id: usize,
    pub chain_type: Type,
    pub material: ChainMaterial,
    pub width: usize,
    pub price: usize,
    pub is_magnit: bool,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub hash: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

