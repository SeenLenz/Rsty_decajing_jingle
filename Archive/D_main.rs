use std::io;
use std::mem::drop;
use std::io::Write;
use std::fs::ReadDir;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

mod file_handling;
mod audio_handling;

fn main() -> io::Result<()> {
    let mut folders: Vec<ReadDir> = Vec::new();
    let mut songs: Vec<file_handling::Needed> = Vec::new();

    loop {
        let mut input: String = String::new();
        print!("Would you like to add/add another path?[Y/N]: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match &input.trim().to_uppercase()[..] {
            "N" => break,
            "Y" => {
                folders.push(file_handling::get_folders()?);
                drop(input);
                continue;
            }
            _   => {
                println!("Invalid input");
                drop(input);
                continue;
            }
        }
    }

    file_handling::parse_folder(folders, &mut songs)?;

    let song = &songs[534]._path;

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output device");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    let file = BufReader::new(File::open(song).expect("Failed to open file"));

    let source = Decoder::new(file).expect("Failed to create a decoder");
    
    sink.append(source);

    sink.sleep_until_end();

    return Ok(());
}
