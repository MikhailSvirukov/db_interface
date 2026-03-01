use core_app::replies::Calculation;
use core_app::requests::SelectedBlock;
use rusqlite::Connection;

pub fn calculate(
    connection: &Connection,
    items: &Vec<SelectedBlock>,
) -> rusqlite::Result<Calculation> {
    let mut sum = 0;
    return Ok((items.len()) as Calculation);
    // let ids = items
    //     .sections
    //     .iter()
    //     .map(|s| s.value)
    //     .collect::<Vec<isize>>();
    //
    // if !ids.is_empty() {
    //     let mut stmt = connection.prepare("SELECT price, id FROM sections")?;
    //     let rows = stmt.query_map(params![], |row| {
    //         let id: isize = row.get(1)?;
    //         let price: isize = row.get(0)?;
    //         Ok((id, price))
    //     })?;
    //
    //     for row in rows {
    //         let (id, price) = row?;
    //         for i in &items.sections {
    //             if i.value == id {
    //                 sum += (i.count as isize) * price;
    //                 break;
    //             }
    //         }
    //     }
    // }
    //
    // let ids = items.chains.iter().map(|s| s.value).collect::<Vec<isize>>();
    // if !ids.is_empty() {
    //     let mut stmt = connection.prepare("SELECT price, id FROM chains")?;
    //
    //     let rows = stmt.query_map(params![], |row| {
    //         let id: isize = row.get(1)?;
    //         let price: isize = row.get(0)?;
    //         Ok((id, price))
    //     })?;
    //
    //     for row in rows {
    //         let (id, price) = row?;
    //         for i in &items.chains {
    //             if i.value == id {
    //                 sum += (i.count as isize) * price;
    //                 break;
    //             }
    //         }
    //     }
    // }
    // Ok(sum)
}
