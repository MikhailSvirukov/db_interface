use crate::types::PipelineType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedBlock {
    pub typ: PipelineType,
    pub section: Id,
    pub chains: Vec<Id>,
    pub accessories: Vec<Id>,
}

pub type Id = isize;
