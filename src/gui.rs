//std imports
use std::error::Error;
use std::fs;
//gui imports
use eframe::{
    egui,
    NativeOptions,
    App,
};
//json imports
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::opint::json_to_string;

#[derive(Serialize, Deserialize)]
pub struct RstyConfig{
    dark_mode: bool,
    main_page: bool,
}

impl RstyConfig{

    pub fn new() -> Result<Self, Box<dyn Error>>{
        //TODO: error_handling
        let json = json_to_string("./config/RstyConfig_cfg.json").unwrap();
        let json_owned = &json[..];
        let cfg: RstyConfig = serde_json::from_str(json_owned).unwrap();
        return Ok(cfg);
    }

    pub fn save(&mut self, dm: bool, mp: bool) -> Result<(), Box<dyn Error>> {

        self.dark_mode = dm;
        self.main_page = mp;
        let modified = serde_json::to_string(self)?;
        fs::write("./config/RstyConfig_cfg.json", modified)?;

        Ok(())
    }
}

impl std::fmt::Display for RstyConfig{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "
        \ndark_mode: {}\nmain_page: {}", 
        self.dark_mode, 
        self.main_page)
    }
}

#[derive(Debug)]
pub struct Song {
    pub id: i32,
    pub img_path: String,
    pub path: String,
    pub name: String,
    pub duration: String,
    pub date_added: String,
    pub clicks: i32,
    pub playlists: Vec<i32>,
}

#[derive(Debug)]
pub struct Playlist<'a> {
    pub id: i32,
    pub img_path: String,
    pub name: String,
    pub clicks: i32,
    pub liked: i32,
    pub date_created: String,
    pub songs: Vec<&'a Song>
    //TODO note: this might... or probably will create bugs when ill work on removing songs, since the lifetime of the playlist might keep a song alive
}


pub struct RstyJingle{
    songs: Vec<Song>,
    cfg: RstyConfig,
}

impl RstyJingle{
    pub fn new() -> Self{

        RstyJingle{
            songs: Vec::<Song>::new(),
            //TODO: error_handling
            cfg: RstyConfig::new().unwrap()
        }
    }

}

fn side_panel(ctx: &egui::Context){

    
}


impl App for RstyJingle{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

    
        
    }

}



