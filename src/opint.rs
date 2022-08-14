use std::fs::{self, DirEntry};
use std::io::Error;
use std::path::PathBuf;
use crate::sql;
use crate::gui::Song;

pub fn json_to_string(path: &str) -> Result<String, Error>{
    return Ok(fs::read_to_string(path)?);
}

pub fn parse_folder(Folders: &Vec<PathBuf>) -> Result<bool, Error>{

    println!("called parse_folder");

    for Folder in Folders{

        let dir = fs::read_dir(Folder)?;

        for entrie in dir{

            let file = entrie?;

            if !file.metadata()?.is_dir(){

                sql::sql_add_song(Song::new_from_entry(file).expect("fuck")).unwrap();
            }
        }
    }
    return Ok(true);
}