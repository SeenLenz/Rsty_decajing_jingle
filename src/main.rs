// use std::collections::VecDeque;
// use std::fmt::format;
// use std::io;
// use std::mem::drop;
// use std::io::Write;
// use std::fs::ReadDir;

// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, Sink};
// use std::path::PathBuf;

// const IMAGE_SIZE: Vec2 = Vec2::new(100.,100.);
#![allow(unused)]

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn sql_init(connection: &Connection) -> Result<()> {

    connection.execute("
    CREATE TABLE IF NOT EXISTS playlists (
        id INTEGER PRIMARY KEY,
        img_path TEXT,
        name TEXT,
        clicks INTEGER,
        liked INTEGER,
        date_created TEXT
    )", NO_PARAMS).expect("creation of playlists failed");
    
    connection.execute("
    INSERT INTO playlists VALUES(1,'./resources/favourites.png','favourites',0,0,'2022.07.11')
    ", NO_PARAMS).expect("creation of favourites failed");

    connection.execute("
    CREATE TABLE IF NOT EXISTS songs (
        id INTEGER PRIMARY KEY,
        path TEXT,
        img_path TEXT,
        name TEXT,
        duration TEXT,
        date_added TEXT,
        clicks INTEGER,
        favourite INTEGER,
        FOREIGN KEY(favourite) REFERENCES playlists(id)
    )", NO_PARAMS).expect("creation of songs failed");

    return Ok(())
}

fn sql_add_song(connection: &Connection, args: (i32, &str, &str, &str, &str, &str, i32, i32,)) -> Result<()> {

    let temp:String = format!("INSERT INTO songs VALUES({},{},{},{},{},{},{}, playlists.id LIMIT 1)", args.0, args.1, args.2, args.3, args.4, args.5, args.6);
    let query = &temp[..]; 
    connection.execute( query,NO_PARAMS)?;

    return Ok(())
}

fn sql_create_playlist(connection: Connection) -> Result<()> {

    return Ok(())
}

fn main() -> Result<()>{

    let conn = Connection::open("./database/rsty_jingle.db").expect("failed to initialise the database");

    sql_init(&conn);
    sql_add_song(&conn,(1, "path", "img_path", "name", "duration", "path", 0, 1));

    return Ok(());
}