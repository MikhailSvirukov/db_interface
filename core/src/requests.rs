use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectionSelect {
    pub value: isize,
    pub count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainSelect {
    pub value: isize,
    pub count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedBlock {
    pub section: Id,
    pub chains: Vec<Id>,
    pub accessories: Vec<Id>,
}

pub type Id = isize;
