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
pub struct SelectedItems {
    pub sections: Vec<SectionSelect>,
    pub chains: Vec<ChainSelect>,
}

pub type Id = isize;
