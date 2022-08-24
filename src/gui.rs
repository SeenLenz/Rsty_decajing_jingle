use std::io::BufReader;
use std::option::Option;
use std::path::PathBuf;
use std::time::SystemTime;
use eframe::egui::SelectableLabel;
use rodio::Source;
use rodio::Decoder;
use rusqlite::Connection;
//std imports
use std::{error::Error};
use std::fs::{self, DirEntry, File};
use rfd::FileDialog;
//gui imports
use eframe::{
    egui,
    App,
    egui::{TopBottomPanel, 
        Vec2,
        SidePanel, 
        style::Visuals, 
        Layout, 
        CentralPanel, 
        Align2,
        Align}
};
//json imports
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::opint::{parse_folder,json_to_string};
use crate::sql;
use crate::sql::sdb_to_vec;
use crate::audio::play;

#[derive(Serialize, Deserialize, Debug)]
pub struct RstyConfig{

    settings_page: bool,
    dark_mode: bool,
    is_linux: bool,
    is_simple: bool,
    pub has_run: bool,
    folders: Vec<PathBuf>
}

impl RstyConfig{

    pub fn new() -> Result<Self, Box<dyn Error>>{
        //TODO: error_handling

        let json = json_to_string("./config/RstyConfig_cfg.json").unwrap();
        let json_owned = &json[..];
        let cfg: RstyConfig = serde_json::from_str(json_owned).unwrap();
        return Ok(cfg);
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {

        let modified = serde_json::to_string(self)?;
        fs::write("./config/RstyConfig_cfg.json", modified)?;

        Ok(())
    }
}

// impl std::fmt::Display for RstyConfig{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "
//         \ndark_mode: {}\nmain_page: {}", 
//         self.dark_mode, 
//         self.main_page)
//     }
// }

#[derive(Debug)]
pub struct Song {
    pub id: i32,
    pub img_path: String,
    pub path: String,
    //pub extension: String,
    pub name: String,
    pub duration: u64,
    pub date_added: String,
    pub clicks: i32,
    pub playlists: Vec<String>,
}

impl Default for Song{

    fn default() -> Self {
        return Song{id:-1,  
            img_path:String::from(""), 
            path: String::from(""), 
            name: String::from(""), 
            duration: 0, 
            date_added: String::from(""), 
            clicks: 0, 
            playlists: Vec::new()
        };

}

}

impl Song{

    pub fn new_from_entry(entry: DirEntry) -> Result<Self, std::io::Error>{

    println!("called Song::new_from_entry");
        
        
        let thing: Song = Song{
            id: 1,
            img_path: "./resources/favourites.png".to_string(),
            path: entry.path().into_os_string().into_string().unwrap(),
            name: entry.file_name().into_string().unwrap(),
            duration: match mp3_duration::from_path(entry.path()) {

                Ok(x) => x.as_secs(),
                Err(x) => 0

            },
            date_added: format!("{:?}",entry.metadata().unwrap().created().unwrap()),
            clicks: 0,
            playlists: Vec::new()};

        println!("{:?}", thing);

        return Ok(thing);
    }
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

#[derive(Debug)]
pub struct RstyJingle{
    pub repaint_after: f32,
    pub songs: Vec<Song>,
    // pub playlists: Vec<Playlist>,
    pub cfg: RstyConfig,
    pub focus: Option<usize>,
}

impl RstyJingle{
    pub fn new() -> Self{

        RstyJingle{
            repaint_after: 1.0,
            songs: Vec::<Song>::new(),
            //TODO: error_handling
            cfg: RstyConfig::new().unwrap(),
            focus: None,
        }
    }

    pub fn refresh(self){}

    pub fn set_focus(mut self, index: usize){

        self.focus = Some(index);

    }
    
}

impl App for RstyJingle{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {



        if self.cfg.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.cfg.has_run == false {

            initgui(&mut self.cfg, ctx, &mut self.songs);

        } else {

            if self.cfg.is_simple {

            } else {

                if self.songs.len() == 0{
                    sdb_to_vec(&mut self.songs).expect("SDB_TO_VEC failed normal startup");
                }

                complex_layout(ctx, &mut self.songs, &mut self.focus, &mut self.cfg);

            }
        }
    }



}

fn complex_layout(ctx: &egui::Context, songs: &mut Vec<Song>, focus: &mut Option<usize>, cfg: &mut RstyConfig){

    bottom_panel(ctx, focus, songs);
    
    side_panel(ctx, cfg);

    if cfg.settings_page {
        settings_page(ctx)
    } else{

        center_panel(ctx, songs, focus);

    }

}

fn trial(ctx: &egui::Context){

    CentralPanel::default().show(ctx, |ui|{

        egui::Window::new("Kek").resize(|r| r.fixed_size(egui::Vec2::new(300., 300.))).show(ctx, |ui|{});

    }); 

}

fn initgui(cfg: &mut RstyConfig, ctx: &egui::Context, songs: &mut Vec<Song>){

    CentralPanel::default().show(ctx, |ui|{

        egui::Window::new("First_time_conf")
        .resizable(false)
        .collapsible(false)
        .hscroll(false)
        .vscroll(false)
        .title_bar(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui|{

            ui.horizontal(|ui|{

                ui.heading("First time Settings");
                
            });

            ui.add_space(30.);

            egui::Grid::new("First_time_conf_table")
            .num_columns(2)
            .striped(true)
            .spacing([0.,30.])
            .show(ui, |ui|{

                    ui.label("Darkmode:");
                    let theme = ui.add(egui::Button::new(format!("{}", cfg.dark_mode)));
                    if theme.clicked(){
                        cfg.dark_mode = !cfg.dark_mode;
                    }
                    ui.end_row();

                    ui.label("Simple Layout:");
                    let layout = ui.add(egui::Button::new(format!("{}", cfg.is_simple)));
                    if layout.clicked(){
                        cfg.is_simple = !cfg.is_simple;
                    }
                    ui.end_row();

                    ui.label("Folders");
                    let add_folder_button = ui.add(egui::Button::new("Add Folder"));
                    if add_folder_button.clicked(){
                        cfg.folders.push(FileDialog::new().pick_folder().expect("Uhf, you fucked up here buddy"));
                    }
                
                    ui.end_row();

            });

            for folder in &cfg.folders{
                ui.label(format!("{}", folder.as_path().display().to_string()));
            }

            
            ui.add_space(30.);
            ui.horizontal(|ui|{

                ui.add_space(50.);
                if ui.add(egui::Button::new("Submit")).clicked() {

                    cfg.has_run = true;

                    sql::sql_init().expect("init sql has failed");

                    parse_folder(&cfg.folders);

                    match fs::read_dir("/home") {
                        Ok(_) => cfg.is_linux = true,
                        Err(_) => cfg.is_linux = false,
                    }

                    cfg.save().expect("saving Rsty config has failed");

                    sdb_to_vec(songs).expect("SDB_TO_VEC failed at startup");

                }
            });
        });
    }); 
}

fn side_panel(ctx: &egui::Context, cfg: &mut RstyConfig){


    SidePanel::left("Options")
        .resizable(false)
        .min_width(250.0)
        .show(ctx,|ui|{

            ui.horizontal(|ui|{
                ui.add_space(10.);
                let Settings= ui.add(egui::Button::new("Settings"));    
                if Settings.clicked(){
                    cfg.settings_page = !cfg.settings_page
                }   
            });

            ui.separator();
            
    });
}

fn bottom_panel(ctx: &egui::Context, focus: &mut Option<usize>, songs: & mut Vec<Song>){
    
    TopBottomPanel::bottom("navbar")
    .min_height(100.0)
    .show(ctx, |ui| {

        // ui.with_layout(Layout::bottom_up(Align::Center), |ui|{
        //     ui.add_space(15.0);
        //     ui.add(egui::ProgressBar::new(0.).desired_width(ui.available_width() / 2.));
        //     ui.add_space(15.0);
        
        //     ui.horizontal(|ui|{

        //         let previous = ui.add(egui::Button::new("⏮"));
        //         let play = ui.add(egui::Button::new("▶"));
        //         let next = ui.add(egui::Button::new("⏭"));

        //     });
        // });

        ui.with_layout(Layout::bottom_up(Align::Center), |ui|{
                
                ui.add_space(15.0);
                ui.add(egui::ProgressBar::new(0.).desired_width(ui.available_width() / 2.));
                ui.add_space(10.0);
                match focus {
                    Some(x) => {
                        ui.label(&songs[focus.unwrap()].name);
                    }
                    None => {
                        ui.label("No song currently playing");
                    }
                }
                ui.add_space(8.0);

            ui.allocate_ui_with_layout(egui::vec2(100., 100.), Layout::bottom_up(Align::Center), |ui|{
            
                ui.horizontal(|ui|{

                    let previous = ui.add(egui::Button::new("⏮"));
                    let play = ui.add(egui::Button::new("▶"));
                    let next = ui.add(egui::Button::new("⏭"));

                });
            });
        });
    });
}
 
fn center_panel(ctx: &egui::Context, songs: & mut Vec<Song>, focus: &mut Option<usize>){

    CentralPanel::default().show(ctx, |ui|{

        egui::ScrollArea::vertical().show(ui, |ui| {

            egui::Grid::new("some_unique_id").striped(true).show(ui, |ui| {


                for song in 0..songs.len(){

                    ui.vertical(|ui|{
                        ui.add_space(20.);
                        ui.horizontal(|ui|{

                            ui.add_space(5.);
                            if ui.link(&songs[song].name).clicked() {
                                println!("current index: {:?}", focus);
                                println!("current song: {}", &songs[song].name);
                                println!("Song id: {}", &songs[song].id);

                                if *focus == Some(song){
                                    *focus = None
                                } else{
                                    *focus = Some(song);
                                }

                                println!("new index: {:?}", focus);
                                println!("new song: {}", &songs[song].name);
                                
                            }
                        });
                        ui.add_space(20.);
                    });
                    ui.end_row();             

                }

            });
            
        });

    }); 

}

fn settings_page(ctx: &egui::Context){

    
}



