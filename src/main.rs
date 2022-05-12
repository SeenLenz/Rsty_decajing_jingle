use std::env::join_paths;
use std::io;
use std::mem::drop;
use std::io::Write;
use std::fs::ReadDir;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

use audio_handling::play_audio;
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

    let song = &songs[374]._path;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(&song).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(15));

    return Ok(());
}
