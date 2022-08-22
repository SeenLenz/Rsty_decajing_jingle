use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;
use crate::gui::{Song, RstyJingle};
use crate::gui::Playlist;

pub fn sql_init() -> Result<()> {

    let connection = Connection::open("./database/rsty_jingle.db")
    .expect("fucked up connection to the database when initilaizing it");

    connection.execute("
    CREATE TABLE IF NOT EXISTS playlists (
        img_path TEXT,
        name TEXT,
        clicks INTEGER,
        liked INTEGER,
        date_created TEXT
    )", NO_PARAMS).expect("creation of playlists failed");
    
    connection.execute("
    INSERT INTO playlists VALUES('./resources/favourites.png','favourites',0,0,'2022.07.11')
    ", NO_PARAMS).expect("creation of favourites failed");

    connection.execute("
    CREATE TABLE IF NOT EXISTS songs (
        img_path TEXT,
        path TEXT,
        name TEXT,
        duration INTEGER,
        date_added TEXT,
        clicks INTEGER,
        playlists TEXT,
        UNIQUE(path)
    )", NO_PARAMS).expect("creation of songs failed");

    return Ok(())
}

pub fn sql_add_song(song: Song) -> Result<()> {
    println!("called sql_add_song");

    //https://stackoverflow.com/questions/57791985/how-to-append-to-existing-text-row-sqlite3
    let connection = Connection::open("./database/rsty_jingle.db")
    .expect("fucked up connection to the database when adding a song");
    connection.execute( "INSERT INTO songs VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![
song.img_path, 
song.path, 
song.name, 
song.duration, 
song.date_added, 
song.clicks, 
song.playlists.join("")
])?;

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

pub fn sdb_to_vec(target_vec: &mut Vec<Song>) -> Result<bool, rusqlite::Error> {

    let connection = Connection::open("./database/rsty_jingle.db")
    .expect("fucked up connection to the database when adding a song");
    let mut revec = Vec::new();
    let mut stmt = connection.prepare("SELECT rowid,  * FROM songs")?;
    let playlist_iter = stmt.query_map([], |r|{
        Ok( target_vec.push(Song {
            id: r.get(0)?,
            img_path: r.get(1)?,
            path: r.get(2)?,
            name: r.get(3)?,
            duration: r.get(4)?,
            date_added: r.get(5)?,
            clicks: r.get(6)?,
            playlists: vec![r.get(7)?],
        }))
    })?;

    for row in playlist_iter{
        revec.push(row?);
    };

    return Ok(true)
}