use crate::default;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

pub async fn open_db() -> rusqlite::Result<Connection> {
    //так себе решение, похоже на костыль, но рабочее
    let flag = Path::new("database.db").exists();
    let connection = Connection::open_with_flags(
        "database.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    init_schema(&connection)?;
    if !flag {
        default(&connection).await;
    }
    Ok(connection)
}

fn init_schema(connection: &Connection) -> rusqlite::Result<()> {
    //create table for sections
    connection.execute(
        "CREATE TABLE IF NOT EXISTS sections (
            id INTEGER PRIMARY KEY,
            pipeline_type INTEGER NOT NULL,
            length INTEGER NOT NULL,
            price INTEGER NOT NULL,
            tags TEXT NOT NULL,
            coef INTEGER NOT NULL,
            opaque TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        (),
    )?;

    //create table for chains
    connection.execute(
        "CREATE TABLE IF NOT EXISTS chains (
            id INTEGER PRIMARY KEY,
            pipeline_type INTEGER NOT NULL,
            material INTEGER NOT NULL,
            price INTEGER NOT NULL,
            name TEXT NOT NULL,
            tags TEXT NOT NULL,
            opaque TEXT NOT NULL
            )",
        (),
    )?;

    //create table for users
    connection.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            hash TEXT NOT NULL,
            email TEXT,
            name TEXT,
            phone TEXT,
            level INTEGER NOT NULL
        )",
        (),
    )?;

    // create table for accessories
    connection.execute(
        "CREATE TABLE IF NOT EXISTS accessories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price INTEGER NOT NULL,
            tags TEXT NOT NULL,
            opaque TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}
