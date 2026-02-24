use rusqlite::Connection;
use std::path::Path;

pub fn open_db() -> rusqlite::Result<Connection> {
    if Path::new("../database.db").exists() {
        Connection::open("../database.db")
    } else {
        create_table()
    }
}

fn create_table() -> rusqlite::Result<Connection> {
    let connection = Connection::open("../database.db")?;
    //create table for sections
    connection.execute(
        "CREATE TABLE sections (
            id INTEGER PRIMARY KEY,
            type INTEGER NOT NULL,
            width INTEGER NOT NULL ,
            length INTEGER NOT NULL ,
            price INTEGER NOT NULL ,
            is_magnet BOOLEAN NOT NULL ,
            material_sides INTEGER,
            radius INTEGER,
            angle INTEGER,
            chains TEXT
        )",
        (),
    )?;

    //create table for chains
    connection.execute(
        "CREATE TABLE chains (
                id INTEGER PRIMARY KEY,
                chain_type INTEGER NOT NULL,
                material INTEGER NOT NULL,
                width INTEGER NOT NULL,
                price INTEGER NOT NULL,
                is_magnet BOOLEAN NOT NULL,
                name TEXT NOT NULL
        )",
        (),
    )?;

    //create table for users
    connection.execute(
        "CREATE TABLE users (
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
        "CREATE TABLE accessories (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
        )",
        (),
    )?;
    Ok(connection)
}
