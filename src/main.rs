use std::io;
use std::mem::drop;
use std::io::Write;
use std::fs::{self, DirEntry, ReadDir};
use std::ffi::{OsStr,OsString};
use std::os::windows::prelude::*;
use std::path::PathBuf;

//========================================= Audio parsing ===========================================
#[allow(dead_code)]
fn folder_to_list(dir_to_path: Vec<PathBuf>, mut result_vec: Vec<Needed>, ) -> std::io::Result<Vec<Needed>> {

    let sup_files: [&str; 3] = ["mp3","wav","ogg"];

    for dir in dir_to_path{

        let dir_struct = fs::read_dir(dir).unwrap();

        for entrie in dir_struct{

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

    return Ok(result_vec);
}

fn get_folders() -> io::Result<Vec<ReadDir>>{
    let mut folders: Vec<ReadDir> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    loop {

    }

    return Ok(folders);
}

#[allow(dead_code)]
fn type_of<T>(_: &T) {
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

//=========================================== File Input =============================================
fn main() -> io::Result<()> {
    
    

    return Ok(());
}



// ========================================= Audio playback ===========================================
// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, source::Source};

// fn main() {

//     // Get a output stream handle to the default physical sound device
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     // Load a sound from a file, using a path relative to Cargo.toml
//     let file = BufReader::new(File::open("src/practice.mp3").unwrap());
//     // Decode that sound file into a source
//     let source = Decoder::new(file).unwrap();
//     // Play the sound directly on the device
//     stream_handle.play_raw(source.convert_samples());

//     // The sound plays in a separate audio thread,
//     // so we need to keep the main thread alive while it's playing.
//     std::thread::sleep(std::time::Duration::from_secs(10));

// }

//=========================================    Archive    ===========================================
// fn main() -> io::Result<()> {
    
//     print!("Please input a path to your music folder: ");
//     io::stdout().flush()?;

//     let mut input = String::new();
//     let mut folders: Vec<String> = Vec::new();
//     io::stdin().read_line(&mut input)?;
//     folders.push(input);

//     loop {
//         let mut countinue = String::new();

//         print!("would you like to input another path?[Y/N]: ");
//         io::stdout().flush()?;

//         io::stdin().read_line(&mut countinue).unwrap();

//         if countinue.trim().to_lowercase().eq("no") || countinue.trim().to_lowercase().eq("n"){

//             break

//         } else {

//             let mut input = String::new();

//             print!("Please input a path to your music folder: ");
//             io::stdout().flush()?;

//             match io::stdin().read_line(&mut input) {

//                 Err(e) => panic!("Error: {}",e),
//                 Ok(_) => {folders.push(input)}
        
//             }
//         }

//         drop(countinue);
//     }

//     println!("Your current folders are:");

//     for i in folders{

//         println!("{}",i);
//     }
//     return Ok(());
// }
