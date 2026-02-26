use crate::{AccessoriesUpdater, ChainUpdater, SectionUpdater, UpdateStatus, UserUpdater};
use core_app::requests::{Id, SelectedBlock};
use core_app::types::{Accessories, Chain, Section, User};

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

pub fn parse_input_section(updater: &mut SectionUpdater) -> Result<Section, String> {
    match updater.section_mode {
        UpdateStatus::Add => {
            Ok(Section {
                //because default
                id: -1,
                section_type: {
                    if updater.section_type.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_type.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                width: {
                    if updater.section_width.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_width.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                length: {
                    if updater.section_lenght.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_lenght.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                price: {
                    if updater.section_price.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_price.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                is_magnet: {
                    if updater.section_is_magnet.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_is_magnet.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                material_sides: {
                    if updater.section_material_sides.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_material_sides.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                radius: {
                    if updater.section_radius.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_radius.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                angle: {
                    if updater.section_angle.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.section_angle.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                chains: {
                    if updater.section_type.is_empty() {
                        return Err("Field can't be empty".to_string());
                    } else {
                        let ids = updater.section_chains.split(",");
                        let mut vec = Vec::new();
                        if updater.section_chains.is_empty() {
                            vec
                        } else {
                            for i in ids {
                                if let Ok(i) = i.parse::<isize>() {
                                    vec.push(i);
                                } else {
                                    return Err("Error fetching dashboard data".to_string());
                                }
                            }
                            vec
                        }
                    }
                },
            })
        }
        UpdateStatus::Update => {
            // here we can use unwrap, since we does have id
            Ok(Section {
                id: updater.section_id.parse().unwrap(),
                section_type: {
                    if let Ok(val) = updater.section_type.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                width: {
                    if let Ok(val) = updater.section_width.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                length: {
                    if let Ok(val) = updater.section_lenght.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                price: {
                    if let Ok(val) = updater.section_price.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                is_magnet: {
                    if let Ok(val) = updater.section_is_magnet.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                material_sides: {
                    if let Ok(val) = updater.section_material_sides.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                radius: {
                    if let Ok(val) = updater.section_radius.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                angle: {
                    if let Ok(val) = updater.section_angle.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                chains: {
                    let ids = updater.section_chains.split(",");
                    let mut vec = Vec::new();
                    if updater.section_chains.is_empty() {
                        vec
                    } else {
                        for i in ids {
                            if let Ok(i) = i.parse::<isize>() {
                                vec.push(i);
                            } else {
                                return Err("Error fetching dashboard data".to_string());
                            }
                        }
                        vec
                    }
                },
            })
        }
        _ => Err("Incorrect state".to_string()),
    }
}

pub fn parse_input_chain(updater: &mut ChainUpdater) -> Result<Chain, String> {
    match updater.section_mode {
        UpdateStatus::Add => {
            Ok(Chain {
                //because default
                id: -1,
                chain_type: {
                    if updater.r#type.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.r#type.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                material: {
                    if updater.material.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.material.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                width: {
                    if updater.width.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.width.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                price: {
                    if updater.price.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.price.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                is_magnet: {
                    if updater.is_magnet.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(val) = updater.is_magnet.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                name: updater.name.clone(),
            })
        }
        UpdateStatus::Update => {
            // here we can use unwrap, since we does have id
            Ok(Chain {
                id: updater.id.parse().unwrap(),
                chain_type: {
                    if let Ok(val) = updater.r#type.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                width: {
                    if let Ok(val) = updater.width.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                material: {
                    if let Ok(val) = updater.material.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                price: {
                    if let Ok(val) = updater.price.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
                is_magnet: {
                    if let Ok(val) = updater.is_magnet.parse() {
                        val
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },

                name: { updater.name.clone() },
            })
        }
        _ => Err("Incorrect State".to_string()),
    }
}

pub fn parse_input_user(updater: &mut UserUpdater) -> Result<User, String> {
    match updater.section_mode {
        UpdateStatus::Add => {
            Ok(User {
                //because default
                id: -1,
                hash: {
                    if updater.hash.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    updater.hash.clone()
                },
                name: {
                    if updater.name.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    updater.name.clone()
                },
                email: {
                    if updater.email.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    updater.email.clone()
                },
                phone: {
                    if updater.phone.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    updater.phone.clone()
                },
                level: {
                    if updater.level.is_empty() {
                        return Err("Field can't be empty".to_string());
                    }
                    if let Ok(value) = updater.level.parse() {
                        value
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
            })
        }
        UpdateStatus::Update => {
            Ok(User {
                // should not fall
                id: updater.id.parse().unwrap(),
                hash: {
                    if updater.hash.is_empty() {
                        return Err("Field can't be empty".to_string());
                    } else {
                        updater.hash.clone()
                    }
                },
                name: {
                    if updater.name.is_empty() {
                        return Err("Field can't be empty".to_string());
                    } else {
                        updater.name.clone()
                    }
                },
                email: {
                    if updater.email.is_empty() {
                        return Err("Field can't be empty".to_string());
                    } else {
                        updater.email.clone()
                    }
                },
                phone: {
                    if updater.phone.is_empty() {
                        return Err("Field can't be empty".to_string());
                    } else {
                        updater.phone.clone()
                    }
                },
                level: {
                    if let Ok(value) = updater.level.parse() {
                        value
                    } else {
                        return Err("Error fetching dashboard data".to_string());
                    }
                },
            })
        }
        _ => Err("Incorrect State".to_string()),
    }
}

pub fn parse_input_accessories(updater: &mut AccessoriesUpdater) -> Result<Accessories, String> {
    match updater.section_mode {
        UpdateStatus::Update => Ok(Accessories {
            id: updater.id.parse().unwrap(),
            name: if updater.name.is_empty() {
                return Err("Field can't be empty".to_string());
            } else {
                updater.name.clone()
            },
        }),
        UpdateStatus::Add => Ok(Accessories {
            id: -1,
            name: if updater.name.is_empty() {
                return Err("Field can't be empty".to_string());
            } else {
                updater.name.clone()
            },
        }),
        _ => Err("Incorrect State".to_string()),
    }
}
pub fn fill_section_updater(section_updater: &mut SectionUpdater, section: &Section) {
    section_updater.section_id = section.id.to_string();
    section_updater.section_angle = section.angle.to_string();
    section_updater.section_type = section.section_type.to_string();
    section_updater.section_material_sides = section.material_sides.to_string();
    section_updater.section_price = section.price.to_string();
    section_updater.section_is_magnet = section.is_magnet.to_string();
    section_updater.section_lenght = section.length.to_string();
    section_updater.section_width = section.width.to_string();
    section_updater.section_radius = section.radius.to_string();
    section_updater.section_chains = section
        .chains
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

pub fn fill_chain_updater(chain_updater: &mut ChainUpdater, chain: &Chain) {
    chain_updater.id = chain.id.to_string();
    chain_updater.price = chain.price.to_string();
    chain_updater.r#type = chain.chain_type.to_string();
    chain_updater.width = chain.width.to_string();
    chain_updater.name = chain.name.clone();
    chain_updater.is_magnet = chain.is_magnet.to_string();
    chain_updater.material = chain.material.to_string();
}

pub fn fill_user_updater(user_updater: &mut UserUpdater, user: &User) {
    user_updater.id = user.id.to_string();
    user_updater.hash = user.hash.clone();
    user_updater.level = user.level.to_string();
    user_updater.phone = user.phone.clone();
    user_updater.email = user.email.clone();
    user_updater.name = user.name.clone();
}

pub fn fill_accessories_updater(
    accessories_updater: &mut AccessoriesUpdater,
    accessories: &Accessories,
) {
    accessories_updater.id = accessories.id.to_string();
    accessories_updater.name = accessories.name.clone();
}
