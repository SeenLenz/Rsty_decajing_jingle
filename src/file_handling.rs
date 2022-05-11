
use std::fmt;
use std::io;
use std::mem::drop;
use std::io::Write;
use std::fs::{self, ReadDir};
use std::ffi::{OsStr,OsString};
use std::os::windows::prelude::*;
use std::path::PathBuf;

pub fn parse_folder(dirs: Vec<ReadDir>, result_vec: &mut Vec<Needed>, ) -> io::Result<()> {
    let sup_files: [&str; 3] = ["mp3","wav","ogg"];


    for dir in dirs{

        for entrie in dir{

            let entrie = entrie?;

            if entrie.metadata()?.is_dir() == false{

                for i in sup_files{

                    if  entrie.path().extension().unwrap() == i {

                        println!("{:?}", entrie.file_name());
                        let thing = Needed::new( 
                        entrie.metadata()?.file_size(), 
                        entrie.file_name(), 
                        entrie.path().extension().unwrap(), 
                        entrie.path());

                        result_vec.push(thing);
                    }
                }
            }
        }
    }

    return Ok(());
}

pub fn get_folders() -> io::Result<ReadDir>{
    

    loop {
        let mut input: String = String::new();
        print!("Please input a path to your music folder: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match fs::read_dir(&input.trim()) {
            Err(error) => {
                println!("{}", error);
                drop(input);
                continue;
            }
            Ok(dir) => {
                return Ok(dir);
            }
        }
    }
}

pub fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
pub struct Needed {
    _size: u64,
    _name: OsString,
    _type: OsString,
    _path: PathBuf,

}

impl Needed{
    pub fn new(_size: u64, _name: OsString, _type: &OsStr, _path: PathBuf,) -> Self{
        Self{
            _size: _size,
            _name: _name,
            _type: _type.to_os_string(),
            _path: _path,
        }
    }
}

impl fmt::Display for Needed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "Size: {}\nName: {:?}\nType: {:?}\nPath: {:?}\n", self._size, self._name, self._type, self._path)
    }
}
