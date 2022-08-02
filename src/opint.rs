use std::fs;
use std::error::Error;

pub fn json_to_string(path: &str) -> Result<String, Box<dyn Error>>{
    return Ok(fs::read_to_string(path)?);
}

pub fn parse_folder(Folders: Vec<String>) -> Result<bool, Box<dyn Error>>{

    for Folder in Folders{
        
        let dir = fs::read_dir(Folder)?;

        for entrie in dir{

            let file = entrie?;

            if file.metadata()?.is_dir() == false{

                if file.path().extension().unwrap() == "mp3"{



                }
            }
        }
    }
    return Ok(true);
}