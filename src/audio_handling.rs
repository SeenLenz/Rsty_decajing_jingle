use std::{fs::File, path::PathBuf};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::io;

pub fn play_raw_audio(_path: &PathBuf, duration: u64) -> io::Result<()>{


    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output device");

    let file = BufReader::new(File::open(_path).expect("Failed to open file"));

    let source = Decoder::new(file).expect("Failed to create a decoder");

    stream_handle.play_raw(source.convert_samples()).expect("Play_raw failed");

    std::thread::sleep(std::time::Duration::from_secs(duration));

    return Ok(());
}
pub fn play_sink(_path: &PathBuf) -> io::Result<()>{

    


    return Ok(());
}