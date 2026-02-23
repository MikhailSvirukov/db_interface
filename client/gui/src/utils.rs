use core_app::requests::Id;
use core_app::types::{Chain, Section};

pub fn get_section_by_id(id: Id, vector: &Vec<Section>) -> Option<&Section> {
    for section in vector {
        if id == section.id {
            return Some(section);
        }
    }
    None
}

pub fn get_chain_by_id(id: Id, vector: &Vec<Chain>) -> Option<&Chain> {
    for chain in vector {
        if id == chain.id {
            return Some(chain);
        }
    }
    None
}
