use crate::types::PipelineType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wheel {
    pub length: usize,
    pub distance: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Lenght {
    None,
    Line(isize),
    //длина + расстояние между роликами
    Wheels(Wheel),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedBlock {
    pub section: Id,
    pub pipeline_type: PipelineType,
    pub length: Lenght,
    pub chains: Vec<Id>,
    pub accessories: Vec<Id>,
}

pub type Id = isize;
