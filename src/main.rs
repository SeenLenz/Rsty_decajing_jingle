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

#[derive(Debug)]
struct Playlist {
    id: i32,
    img_path: String,
    name: String,
    clicks: i32,
    liked: i32,
    date_created: String,
}

#[derive(Debug)]
struct Song {
    id: i32,
    path: String,
    img_path:String,
    name: String,
    duration: String,
    date_added: String,
    clicks: i32,
    favourite: i32,
}

use rusqlite::{Connection, Result, params, Statement};
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

    connection.execute( "INSERT INTO songs VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7])?;

    return Ok(())
}

fn sql_create_playlist(connection: &Connection, args: (i32, &str, &str, i32, i32, &str)) -> Result<()> {

    connection.execute( "INSERT INTO playlists VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
    params![args.0, args.1, args.2, args.3, args.4, args.5])?;
    return Ok(())
}

fn sql_query_playlist(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

    let data = Playlist{
        id: 0,
        img_path: "/default/path".to_string(),
        name: "test_playlist".to_string(),
        clicks: 0,
        liked: 0,
        date_created: "2022.07.12 09:39".to_string(), 
    };
    let mut stmt = connection.prepare("SELECT * FROM playlists WHERE id = ?1")?;
    let playlist_iter = stmt.query_map([id], |r|{
        Ok( Playlist {
            id: r.get(0)?,
            img_path: r.get(1)?,
            name: r.get(2)?,
            clicks: r.get(3)?,
            liked: r.get(4)?,
            date_created: r.get(5)?,
        })
    })?;

    for row in playlist_iter{
        println!("playlist retrieved with id of [{}]: {:?}", id, row?);
    };

    return Ok(())
}

fn sql_query_song(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

    let data = Song {
        id: 0,
        path: "".to_string(),
        img_path: "".to_string(),
        name: "".to_string(),
        duration: "".to_string(),
        date_added: "".to_string(),
        clicks: 0,
        favourite: 0,
    };
    let mut stmt = connection.prepare("SELECT * FROM songs WHERE id = ?1")?;
    let playlist_iter = stmt.query_map([id], |r|{
        Ok( Song {
            id: r.get(0)?,
            path: r.get(1)?,
            img_path: r.get(2)?,
            name: r.get(3)?,
            duration: r.get(4)?,
            date_added: r.get(5)?,
            clicks: r.get(6)?,
            favourite: r.get(7)?,
        })
    })?;

    for row in playlist_iter{
        println!("playlist retrieved with id of [{}]: {:?}", id, row?);
    };

    return Ok(())
}


fn main() -> Result<()>{

    let conn = Connection::open("./database/rsty_jingle.db").expect("failed to initialise the database");

    // sql_init(&conn);
    // sql_add_song(&conn,(1, "path", "img_path", "name", "duration", "date_added", 0, 1));
    // sql_create_playlist(&conn,(2, "img_path", "name", 0, 0, "creation_date"));
    // println!("{:?}",sql_query_playlist(&conn, 1));
    println!("{:?}",sql_query_song(&conn, 1));

    return Ok(());
}