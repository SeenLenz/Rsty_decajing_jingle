use std::path::PathBuf;
use rusqlite::Connection;
//std imports
use std::{error::Error};
use std::fs;
use rfd::FileDialog;
//gui imports
use eframe::{
    egui,
    App,
    egui::{TopBottomPanel, 
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
use crate::opint::json_to_string;
use crate::sql;

#[derive(Serialize, Deserialize, Debug)]
pub struct RstyConfig{
    dark_mode: bool,
    main_page: bool,
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

#[derive(Debug)]
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

    pub fn refresh(self){}
    
}

impl App for RstyJingle{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if self.cfg.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.cfg.has_run == false {

            initgui(&mut self.cfg, ctx);

        } else {
    
            bottom_panel(ctx);
    
            side_panel(ctx);
    
            center_panel(ctx);

        }
    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

}

fn initgui(cfg: &mut RstyConfig, ctx: &egui::Context){

    CentralPanel::default().show(ctx, |ui|{

        egui::Window::new("First_time_conf")

        .resizable(false)
        .collapsible(false)
        .default_width(500.)
        .default_height(500.)
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

                    let connection = Connection::open("./database/rsty_jingle.db").expect("fucked up connection to the database");

                    sql::sql_init(&connection).expect("init sql has failed");

                    match fs::read_dir("/home") {
                        Ok(_) => cfg.is_linux = true,
                        Err(_) => cfg.is_linux = false,
                    }

                    cfg.save().expect("saving Rsty config has failed");

                }
            });
        });
    }); 
}

fn side_panel(ctx: &egui::Context){

    SidePanel::left("Options")
        .resizable(false)
        .min_width(250.0)
        .show(ctx,|ui|{

            ui.label("Options/search");
            let theme_changer = ui.add(egui::Button::new("🌙"));
            let page_changer = ui.add(egui::Button::new("🎵"));
    });
}

fn bottom_panel(ctx: &egui::Context){
    
    TopBottomPanel::bottom("navbar")
    .min_height(100.0)
    .show(ctx, |ui| {

        ui.label("Player");
        ui.with_layout(Layout::bottom_up(Align::Center), |ui|{
            ui.add_space(15.0);
            ui.add(egui::ProgressBar::new(120.0).desired_width(ui.available_width() / 2.))
        });
    });
    
}

fn center_panel(ctx: &egui::Context){

    CentralPanel::default().show(ctx, |ui|{

        ui.label("something else");
        
    }); 
    
}

fn main_page(ctx: &egui::Context){

    
}

fn settings_page(ctx: &egui::Context){

    
}