use rusqlite::Connection;

fn delete(connection: &Connection, vector: &Vec<isize>, name: &str) -> rusqlite::Result<usize> {
    if vector.is_empty() {
        return Ok(0);
    }
    let placeholders: Vec<String> = (0..vector.len()).map(|_| "?".to_string()).collect();
    let placeholders = placeholders.join(",");

    let sql = format!("DELETE FROM {} WHERE id IN ({})", name, placeholders);

    let mut stmt = connection.prepare(&sql)?;
    let rows_affected = stmt.execute(rusqlite::params_from_iter(vector))?;
    Ok(rows_affected)
}

pub fn delete_section(connection: &Connection, section: &Vec<isize>) -> rusqlite::Result<usize> {
    delete(connection, section, "sections")
}

pub fn delete_chain(connection: &Connection, chain: &Vec<isize>) -> rusqlite::Result<usize> {
    delete(connection, chain, "chains")
}

pub fn delete_user(connection: &Connection, user: &Vec<isize>) -> rusqlite::Result<usize> {
    delete(connection, user, "users")
}
