use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;
use crate::gui::Song;
use crate::gui::Playlist;

pub fn sql_init(connection: &Connection) -> Result<()> {

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
        playlists TEXT,
        UNIQUE(path)
    )", NO_PARAMS).expect("creation of songs failed");

    return Ok(())
}

fn sql_add_song(connection: &Connection, song: &Song) -> Result<()> {

    //https://stackoverflow.com/questions/57791985/how-to-append-to-existing-text-row-sqlite3
    connection.execute( "INSERT OR IGNORE INTO songs VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![
    song.id, 
    song.img_path, 
    song.path, 
    song.name, 
    song.duration, 
    song.date_added, 
    song.clicks, "1"])?;

    return Ok(())
}

fn sql_create_playlist(connection: &Connection, args: (i32, &str, &str, i32, i32, &str)) -> Result<()> {

    connection.execute( "INSERT INTO playlists VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
    params![args.0, args.1, args.2, args.3, args.4, args.5])?;
    return Ok(())
}

fn sql_query_playlist(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

    let mut stmt = connection.prepare("SELECT * FROM playlists WHERE id = ?1")?;
    let playlist_iter = stmt.query_map([id], |r|{
        Ok( Playlist {
            id: r.get(0)?,
            img_path: r.get(1)?,
            name: r.get(2)?,
            clicks: r.get(3)?,
            liked: r.get(4)?,
            date_created: r.get(5)?,
            songs: Vec::new()
        })
    })?;

    return Ok(())
}

fn sql_query_song(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

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
            playlists: Vec::new(),
        })
    })?;

    for row in playlist_iter{
        println!("playlist retrieved with id of [{}]: {:?}", id, row?);
    };

    return Ok(())
}