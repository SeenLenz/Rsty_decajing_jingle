// pub mod gui{

//     use egui_extras::image::RetainedImage;
//     use eframe::{egui,
//     egui::CentralPanel, 
//     App, 
//     egui::{TopBottomPanel, SidePanel, style::Visuals, ScrollArea, Layout}
//     };

//     const PADDING: f32 = 50.;
//     pub struct SongCard {
//         img: RetainedImage,
//         path: String,
//         lenght: String,
//         cfg: String,
//     }

//     #[derive(Default)]
//     pub struct RstyJingle{
//         songs: Vec<SongCard>
//     }

//     impl RstyJingle {

//         pub fn new() -> RstyJingle{

//             let iter =(0..20).map(|a| SongCard{
//                 img: RetainedImage::from_image_bytes("1.png",include_bytes!("1.png")).unwrap(),
//                 path: format!("/home/lenz/Music{}",a),
//                 lenght: format!("{}{}:{}",a,a,a),
//                 cfg: format!("{}",a),
//             });

//             RstyJingle {
//                 songs:Vec::from_iter(iter)
//             }
//         }
//     }


//     impl App for RstyJingle {

//         fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//             let fekete_pako = Visuals {dark_mode: true, ..Default::default()};
//             egui::Context::set_visuals(ctx, fekete_pako);

//             TopBottomPanel::bottom("navbar").min_height(100.0).max_height(150.0).show(ctx, |ui| {
//                 ui.label("Player");
//                 ui.add(egui::ProgressBar::new(0.0))
//             });

//             SidePanel::right("Options").resizable(false).min_width(200.0).show(ctx,|ui|{
//                 ui.label("Options/search");
//                 ui.add(egui::Button::new("🌙")).clicked()
//             });

//             CentralPanel::default().show(ctx, |ui|{

//                 ScrollArea::vertical().auto_shrink([false,false]).show(ui, |ui|{

//                     for song in &self.songs{
//                         ui.with_layout(Layout::right_to_left(), |ui|{
//                             song.img.show(ui);
//                             ui.label(song.path.clone());
//                             ui.label(song.lenght.clone());
//                             ui.label(song.cfg.clone());

//                         });
//                         ui.add_space(PADDING);
//                     }
//                 });
//             });
//         }

//         fn on_exit_event(&mut self) -> bool {
//             true
//         }

//         fn on_exit(&mut self, _gl: &eframe::glow::Context) {}
//     }

//     // use eframe::{egui,
//     //     run_native, 
//     //     NativeOptions,
//     // };
    
//     // mod file_handling;
//     // mod audio_handling;
//     // pub mod lib;
    
    
//     // fn main(){
        
        
    
//     //     let app = gui::RstyJingle::new();
//     //     let win_options = NativeOptions {
//     //         initial_window_size: Some(egui::vec2(960.0, 960.0)),
//     //         ..Default::default()
//     //     };
    
//     //     run_native("rsty_jingle", win_options, Box::new(|cc| Box::new(RstyJingle::new())));
    
//     // }
    
// }

// pub mod file_handling {

//     use std::fmt;
//     use std::io;
//     use std::mem::drop;
//     use std::io::Write;
//     use std::fs::{self, ReadDir};
//     use std::ffi::{OsStr,OsString};
//     use std::path::PathBuf;


//     pub fn parse_folder(dirs: Vec<ReadDir>, result_vec: &mut Vec<Needed>, ) -> io::Result<()> {
//         let sup_files: [&str; 3] = ["mp3","wav","ogg"];

//         for dir in dirs{
//             for entrie in dir{
//                 let entrie = entrie?;
//                 if entrie.metadata()?.is_dir() == false{
//                     for i in sup_files{
//                         match entrie.path().extension() {
//                             Some(value) => {
//                                 if value == i {
                                    
//                                     let thing = Needed::new(
//                                     entrie.file_name(), 
//                                     entrie.path().extension().expect("number two panicked"), 
//                                     entrie.path());
                                    
//                                     result_vec.push(thing);
//                                 }
//                             }

//                             None => {
//                                 println!("The file type could not be determined");
//                                 continue;
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         return Ok(());
//     }

//     pub fn get_folders() -> io::Result<ReadDir>{
//         loop {
//             let mut input: String = String::new();
//             print!("Please input a path to your music folder: ");
//             io::stdout().flush()?;
//             io::stdin().read_line(&mut input)?;


//             match fs::read_dir(&input.trim()) {
//                 Err(error) => {
//                     println!("{}", error);
//                     drop(input);
//                     continue;
//                 }
//                 Ok(dir) => {
//                     return Ok(dir);
//                 }
//             }
//         }
//     }

//     pub fn type_of<T>(_: &T) {
//         println!("{}", std::any::type_name::<T>())
//     }

//     #[derive(Debug)]
//     pub struct Needed {
//         pub _name: OsString,
//         pub _type: OsString,
//         pub _path: PathBuf,

//     }

//     impl Needed{
//         pub fn new(_name: OsString, _type: &OsStr, _path: PathBuf,) -> Self{
//             Self{
//                 _name: _name,
//                 _type: _type.to_os_string(),
//                 _path: _path,
//             }
//         }
//     }

//     impl fmt::Display for Needed {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             // Customize so only `x` and `y` are denoted.
//             write!(f, "Name: {:?}\nType: {:?}\nPath: {:?}\n",  
//                     self._name, 
//                     self._type, 
//                     self._path)
//         }
        
// }


// }

// pub mod audio_handling {

//     use std::{fs::File, path::PathBuf};
//     use std::io::BufReader;
//     use rodio::{Decoder, OutputStream, source::Source};
//     use std::io;

//     pub fn play_raw_audio(_path: &PathBuf, duration: u64) -> io::Result<()>{


//         let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output device");

//         let file = BufReader::new(File::open(_path).expect("Failed to open file"));

//         let source = Decoder::new(file).expect("Failed to create a decoder");

//         stream_handle.play_raw(source.convert_samples()).expect("Play_raw failed");

//         std::thread::sleep(std::time::Duration::from_secs(duration));

//         return Ok(());
//     }
//     pub fn play_sink(_path: &PathBuf) -> io::Result<()>{

        


//         return Ok(());
// }

// }

// pub mod d_main {

//     // use std::io;
//     // use std::mem::drop;
//     // use std::io::Write;
//     // use std::fs::ReadDir;

//     // use std::fs::File;
//     // use std::io::BufReader;
//     // use rodio::{Decoder, OutputStream, Sink};

//     // mod file_handling;
//     // mod audio_handling;

//     // fn main() -> io::Result<()> {
//     //     let mut folders: Vec<ReadDir> = Vec::new();
//     //     let mut songs: Vec<file_handling::Needed> = Vec::new();

//     //     loop {
//     //         let mut input: String = String::new();
//     //         print!("Would you like to add/add another path?[Y/N]: ");
//     //         io::stdout().flush()?;
//     //         io::stdin().read_line(&mut input)?;

//     //         match &input.trim().to_uppercase()[..] {
//     //             "N" => break,
//     //             "Y" => {
//     //                 folders.push(file_handling::get_folders()?);
//     //                 drop(input);
//     //                 continue;
//     //             }
//     //             _   => {
//     //                 println!("Invalid input");
//     //                 drop(input);
//     //                 continue;
//     //             }
//     //         }
//     //     }

//     //     file_handling::parse_folder(folders, &mut songs)?;

//     //     let song = &songs[534]._path;

//     //     let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output device");
//     //     let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

//     //     let file = BufReader::new(File::open(song).expect("Failed to open file"));

//     //     let source = Decoder::new(file).expect("Failed to create a decoder");
        
//     //     sink.append(source);

//     //     sink.sleep_until_end();

//     //     return Ok(());
//     // }

// }
