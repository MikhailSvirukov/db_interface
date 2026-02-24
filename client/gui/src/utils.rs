use core_app::requests::{Id, SelectedBlock};
use core_app::types::{Accessories, Chain, Section};

pub fn get_section_by_id(id: Id, vector: &Vec<Section>) -> Option<&Section> {
    for section in vector {
        if id == section.id {
            return Some(section);
        }
    }
    None
}

pub fn remove_selected_block(id: usize, vector: &mut Vec<SelectedBlock>) {
    vector.remove(id);
}

pub fn get_chain_by_id(id: Id, vector: &Vec<Chain>) -> Option<&Chain> {
    for chain in vector {
        if id == chain.id {
            return Some(chain);
        }
    }
    None
}

pub fn get_accessories_by_id(id: Id, vector: &Vec<Accessories>) -> Option<&Accessories> {
    for acc in vector {
        if id == acc.id {
            return Some(acc);
        }
    }
    None
}

pub fn remove_selected_by_id(id: Id, vector: &mut Vec<Id>) {
    for u in 0..vector.len() {
        if id == vector[u] {
            vector.remove(u);
            break;
        }
    }
}
