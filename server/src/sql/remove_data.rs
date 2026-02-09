use core_app::types::{Chain, Section, User};
use rusqlite::Connection;

pub fn delete_section(connection: &Connection, section: &Section) -> rusqlite::Result<usize> {
    connection.execute("DELETE FROM sections AND id = ?1", [section.id])
}

pub fn delete_chain(connection: &Connection, chain: &Chain) -> rusqlite::Result<usize> {
    connection.execute("DELETE FROM sections AND id = ?1", [chain.id])
}

pub fn delete_user(connection: &Connection, user: &User) -> rusqlite::Result<usize> {
    connection.execute("DELETE FROM sections AND id = ?1", [user.id])
}
