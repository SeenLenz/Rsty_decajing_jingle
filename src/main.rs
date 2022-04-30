// use std::io;
// use std::mem::drop;
// use std::fs::{DirEntry};

// // fn type_of<T>(_: &T) {
// //     println!("{}", std::any::type_name::<T>())
// // }

// fn main() -> io::Result<()> {
//     println!("Please input a path to your music folder: ");

//     let mut input = String::new();
//     let mut folders: Vec<String> = Vec::new();
//     let files: Vec<DirEntry> = Vec::new();

//     match io::stdin().read_line(&mut input) {

//         Err(e) => panic!("Error: {}",e),
//         Ok(_) => {folders.push(input)}

//     }

//     loop {
//         let mut countinue = String::new();

//         println!("would you like to input another path?[Y/N]: ");

//         io::stdin().read_line(&mut countinue).unwrap();

//         if countinue.trim().to_lowercase().eq("no") || countinue.trim().to_lowercase().eq("n"){

//             break

//         } else {

//             let mut input = String::new();

//             println!("Please input a path to your music folder: ");

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
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("examples/music.ogg").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

}





