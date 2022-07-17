pub mod gui{

    use egui_extras::image::RetainedImage;
    use eframe::{egui,
    egui::CentralPanel, 
    App, 
    egui::{TopBottomPanel, SidePanel, style::Visuals, ScrollArea, Layout}
    };

    const PADDING: f32 = 50.;
    pub struct SongCard {
        img: RetainedImage,
        path: String,
        lenght: String,
        cfg: String,
    }

    #[derive(Default)]
    pub struct RstyJingle{
        songs: Vec<SongCard>
    }

    impl RstyJingle {

        pub fn new() -> RstyJingle{

            let iter =(0..20).map(|a| SongCard{
                img: RetainedImage::from_image_bytes("1.png",include_bytes!("1.png")).unwrap(),
                path: format!("/home/lenz/Music{}",a),
                lenght: format!("{}{}:{}",a,a,a),
                cfg: format!("{}",a),
            });

            RstyJingle {
                songs:Vec::from_iter(iter)
            }
        }
    }


    impl App for RstyJingle {

        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            let fekete_pako = Visuals {dark_mode: true, ..Default::default()};
            egui::Context::set_visuals(ctx, fekete_pako);

            TopBottomPanel::bottom("navbar").min_height(100.0).max_height(150.0).show(ctx, |ui| {
                ui.label("Player");
                ui.add(egui::ProgressBar::new(0.0))
            });

            SidePanel::right("Options").resizable(false).min_width(200.0).show(ctx,|ui|{
                ui.label("Options/search");
                ui.add(egui::Button::new("🌙")).clicked()
            });

            CentralPanel::default().show(ctx, |ui|{

                ScrollArea::vertical().auto_shrink([false,false]).show(ui, |ui|{

                    for song in &self.songs{
                        ui.with_layout(Layout::right_to_left(), |ui|{
                            song.img.show(ui);
                            ui.label(song.path.clone());
                            ui.label(song.lenght.clone());
                            ui.label(song.cfg.clone());

                        });
                        ui.add_space(PADDING);
                    }
                });
            });
        }

        fn on_exit_event(&mut self) -> bool {
            true
        }

        fn on_exit(&mut self, _gl: &eframe::glow::Context) {}
    }

    // use eframe::{egui,
    //     run_native, 
    //     NativeOptions,
    // };
    
    // mod file_handling;
    // mod audio_handling;
    // pub mod lib;
    
    
    // fn main(){
        
        
    
    //     let app = gui::RstyJingle::new();
    //     let win_options = NativeOptions {
    //         initial_window_size: Some(egui::vec2(960.0, 960.0)),
    //         ..Default::default()
    //     };
    
    //     run_native("rsty_jingle", win_options, Box::new(|cc| Box::new(RstyJingle::new())));
    
    // }
    
}

pub mod file_handling {

    use std::fmt;
    use std::io;
    use std::mem::drop;
    use std::io::Write;
    use std::fs::{self, ReadDir};
    use std::ffi::{OsStr,OsString};
    use std::path::PathBuf;


    pub fn parse_folder(dirs: Vec<ReadDir>, result_vec: &mut Vec<Needed>, ) -> io::Result<()> {
        let sup_files: [&str; 3] = ["mp3","wav","ogg"];

        for dir in dirs{
            for entrie in dir{
                let entrie = entrie?;
                if entrie.metadata()?.is_dir() == false{
                    for i in sup_files{
                        match entrie.path().extension() {
                            Some(value) => {
                                if value == i {
                                    
                                    let thing = Needed::new(
                                    entrie.file_name(), 
                                    entrie.path().extension().expect("number two panicked"), 
                                    entrie.path());
                                    
                                    result_vec.push(thing);
                                }
                            }

                            None => {
                                println!("The file type could not be determined");
                                continue;
                            }
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
        pub _name: OsString,
        pub _type: OsString,
        pub _path: PathBuf,

    }

    impl Needed{
        pub fn new(_name: OsString, _type: &OsStr, _path: PathBuf,) -> Self{
            Self{
                _name: _name,
                _type: _type.to_os_string(),
                _path: _path,
            }
        }
    }

    impl fmt::Display for Needed {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "Name: {:?}\nType: {:?}\nPath: {:?}\n",  
                    self._name, 
                    self._type, 
                    self._path)
        }
        
}


}

pub mod audio_handling {

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

}

pub mod d_main {

    // use std::io;
    // use std::mem::drop;
    // use std::io::Write;
    // use std::fs::ReadDir;

    // use std::fs::File;
    // use std::io::BufReader;
    // use rodio::{Decoder, OutputStream, Sink};

    // mod file_handling;
    // mod audio_handling;

    // fn main() -> io::Result<()> {
    //     let mut folders: Vec<ReadDir> = Vec::new();
    //     let mut songs: Vec<file_handling::Needed> = Vec::new();

    //     loop {
    //         let mut input: String = String::new();
    //         print!("Would you like to add/add another path?[Y/N]: ");
    //         io::stdout().flush()?;
    //         io::stdin().read_line(&mut input)?;

    //         match &input.trim().to_uppercase()[..] {
    //             "N" => break,
    //             "Y" => {
    //                 folders.push(file_handling::get_folders()?);
    //                 drop(input);
    //                 continue;
    //             }
    //             _   => {
    //                 println!("Invalid input");
    //                 drop(input);
    //                 continue;
    //             }
    //         }
    //     }

    //     file_handling::parse_folder(folders, &mut songs)?;

    //     let song = &songs[534]._path;

    //     let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output device");
    //     let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    //     let file = BufReader::new(File::open(song).expect("Failed to open file"));

    //     let source = Decoder::new(file).expect("Failed to create a decoder");
        
    //     sink.append(source);

    //     sink.sleep_until_end();

    //     return Ok(());
    // }

}

mod database_handling {

    #[derive(Debug)]
struct Playlist {
    id: i32,
    img_path: String,
    name: String,
    clicks: i32,
    liked: i32,
    date_created: String,
}

#[derive(Debug)]
struct Song {
    id: i32,
    path: String,
    img_path:String,
    name: String,
    duration: String,
    date_added: String,
    clicks: i32,
    favourite: i32,
}

use rusqlite::{Connection, Result, params, Statement};
use rusqlite::NO_PARAMS;

fn sql_init(connection: &Connection) -> Result<()> {

    connection.execute("
    CREATE TABLE IF NOT EXISTS playlists (
        id INTEGER PRIMARY KEY,
        img_path TEXT,
        name TEXT,
        clicks INTEGER,
        liked INTEGER,
        date_created TEXT
    )", NO_PARAMS).expect("creation of playlists failed");
    
    connection.execute("
    INSERT INTO playlists VALUES(1,'./resources/favourites.png','favourites',0,0,'2022.07.11')
    ", NO_PARAMS).expect("creation of favourites failed");

    connection.execute("
    CREATE TABLE IF NOT EXISTS songs (
        id INTEGER PRIMARY KEY,
        path TEXT,
        img_path TEXT,
        name TEXT,
        duration TEXT,
        date_added TEXT,
        clicks INTEGER,
        favourite INTEGER,
        FOREIGN KEY(favourite) REFERENCES playlists(id)
    )", NO_PARAMS).expect("creation of songs failed");

    return Ok(())
}

fn sql_add_song(connection: &Connection, args: (i32, &str, &str, &str, &str, &str, i32, i32,)) -> Result<()> {

    connection.execute( "INSERT INTO songs VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7])?;

    return Ok(())
}

fn sql_create_playlist(connection: &Connection, args: (i32, &str, &str, i32, i32, &str)) -> Result<()> {

    connection.execute( "INSERT INTO playlists VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
    params![args.0, args.1, args.2, args.3, args.4, args.5])?;
    return Ok(())
}

fn sql_query_playlist(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

    let data = Playlist{
        id: 0,
        img_path: "/default/path".to_string(),
        name: "test_playlist".to_string(),
        clicks: 0,
        liked: 0,
        date_created: "2022.07.12 09:39".to_string(), 
    };
    let mut stmt = connection.prepare("SELECT * FROM playlists WHERE id = ?1")?;
    let playlist_iter = stmt.query_map([id], |r|{
        Ok( Playlist {
            id: r.get(0)?,
            img_path: r.get(1)?,
            name: r.get(2)?,
            clicks: r.get(3)?,
            liked: r.get(4)?,
            date_created: r.get(5)?,
        })
    })?;

    for row in playlist_iter{
        println!("playlist retrieved with id of [{}]: {:?}", id, row?);
    };

    return Ok(())
}

fn sql_query_song(connection: &Connection, id: i32) -> Result<(), rusqlite::Error> {

    let data = Song {
        id: 0,
        path: "".to_string(),
        img_path: "".to_string(),
        name: "".to_string(),
        duration: "".to_string(),
        date_added: "".to_string(),
        clicks: 0,
        favourite: 0,
    };
    let mut stmt = connection.prepare("SELECT * FROM songs WHERE id = ?1")?;
    let playlist_iter = stmt.query_map([id], |r|{
        Ok( Song {
            id: r.get(0)?,
            path: r.get(1)?,
            img_path: r.get(2)?,
            name: r.get(3)?,
            duration: r.get(4)?,
            date_added: r.get(5)?,
            clicks: r.get(6)?,
            favourite: r.get(7)?,
        })
    })?;

    for row in playlist_iter{
        println!("playlist retrieved with id of [{}]: {:?}", id, row?);
    };

    return Ok(())
}

// let conn = Connection::open("./database/rsty_jingle.db").expect("failed to initialise the database");
// sql_init(&conn);
// sql_add_song(&conn,(1, "path", "img_path", "name", "duration", "date_added", 0, 1));
// sql_create_playlist(&conn,(2, "img_path", "name", 0, 0, "creation_date"));
// println!("{:?}",sql_query_playlist(&conn, 1));
// println!("{:?}",sql_query_song(&conn, 1));


}