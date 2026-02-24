use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedBlock {
    pub section: Id,
    pub chains: Vec<Id>,
    pub accessories: Vec<Id>,
}

pub type Id = isize;
