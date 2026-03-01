use crate::sql;
use core_app::replies::Calculation;
use core_app::requests::{Lenght, SelectedBlock};
use core_app::types::PipelineType;
use num::ToPrimitive;
use rusqlite::Connection;
use std::ops::Div;

const COEFFICIENT: isize = 2;

pub fn calculate(
    connection: &Connection,
    items: &Vec<SelectedBlock>,
) -> rusqlite::Result<Calculation> {
    let mut sum = 0;
    for item in items {
        let section = sql::get_data::get_section_by_id(connection, item.section)?;
        let strategy = section.pipeline_type;
        let section_price = match strategy {
            PipelineType::Madal => {
                let width = match &item.length {
                    Lenght::Line(n) => n,
                    _ => {
                        return Err(rusqlite::Error::InvalidQuery);
                    }
                };
                section.price * COEFFICIENT * section.length * width
            }
            PipelineType::Lamellar => section.price * COEFFICIENT * section.length,
            PipelineType::Rolgang => {
                let width = match &item.length {
                    Lenght::Wheels(wh) => wh.length,
                    _ => {
                        return Err(rusqlite::Error::InvalidQuery);
                    }
                } as f64;
                let dist = match &item.length {
                    Lenght::Wheels(wh) => {
                        if wh.distance == 0 {
                            return Err(rusqlite::Error::InvalidQuery);
                        }
                        1_f64.div(wh.distance.to_f64().unwrap())
                    }
                    _ => {
                        return Err(rusqlite::Error::InvalidQuery);
                    }
                };
                (width * dist * (section.length as f64) * (section.price as f64))
                    .to_isize()
                    .unwrap()
            }
            PipelineType::None => {
                return Err(rusqlite::Error::InvalidQuery);
            }
        };
        //TODO: check about compatibility
        sum += section_price;
        for chain in &item.chains {
            let chain = sql::get_data::get_chain_by_id(connection, *chain)?;
            //TODO: price in chain in considered as if
            sum += chain.price;
        }
        //TODO: check about compatibility
        for acc in &item.accessories {
            let accessories = sql::get_data::get_accessories_by_id(connection, *acc)?;
            //TODO: price of accessories in considered as if
            sum += accessories.price;
        }
    }
    Ok(sum)
}
