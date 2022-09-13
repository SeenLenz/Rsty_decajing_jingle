use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

use eframe::egui::SelectableLabel;
use egui::style::DebugOptions;
use rusqlite::Connection;
use std::option::Option;
use std::path::PathBuf;
use std::time::SystemTime;
//std imports
use rfd::FileDialog;
use std::error::Error;
use std::fs::{self, DirEntry};
//gui imports
use eframe::{
    egui,
    egui::{style::Visuals, Align, Align2, CentralPanel, Layout, SidePanel, TopBottomPanel, Vec2},
    App,
};
//json imports
use crate::audio::play;
use crate::opint::{json_to_string, parse_folder};
use crate::sql;
use crate::sql::sdb_to_vec;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct RstyConfig {
    pub debug: bool,
    pub settings_page: bool,
    pub dark_mode: bool,
    pub is_linux: bool,
    pub is_simple: bool,
    pub has_run: bool,
    pub folders: Vec<PathBuf>,
}

impl Default for RstyConfig {
    fn default() -> Self {
        RstyConfig {
            debug: false,
            settings_page: false,
            dark_mode: true,
            is_linux: false,
            is_simple: false,
            has_run: false,
            folders: Vec::new(),
        }
    }
}

impl RstyConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
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

impl Default for Song {
    fn default() -> Self {
        return Song {
            id: -1,
            img_path: String::from(""),
            path: String::from(""),
            name: String::from(""),
            duration: 0,
            date_added: String::from(""),
            clicks: 0,
            playlists: Vec::new(),
        };
    }
}

impl Song {
    pub fn new_from_entry(entry: DirEntry) -> Result<Self, std::io::Error> {
        println!("called Song::new_from_entry");

        let thing: Song = Song {
            id: 1,
            img_path: "./resources/favourites.png".to_string(),
            path: entry.path().into_os_string().into_string().unwrap(),
            name: entry.file_name().into_string().unwrap(),
            duration: match mp3_duration::from_path(entry.path()) {
                Ok(x) => x.as_secs(),
                Err(x) => 0,
            },
            date_added: format!("{:?}", entry.metadata().unwrap().created().unwrap()),
            clicks: 0,
            playlists: Vec::new(),
        };

        println!("{:?}", thing);

        return Ok(thing);
    }

    pub fn play_shitty(&mut self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(&self.path).unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_secs(69));
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
    pub songs: Vec<&'a Song>, //TODO note: this might... or probably will create bugs when ill work on removing songs, since the lifetime of the playlist might keep a song alive
}

#[derive(Debug)]
pub struct RstyJingle {
    pub repaint_after: f32,
    pub songs: Vec<Song>,
    // pub playlists: Vec<Playlist>,
    pub cfg: RstyConfig,
    pub focus: Option<usize>,
}

impl RstyJingle {
    pub fn new() -> Self {
        RstyJingle {
            repaint_after: 1.0,
            songs: Vec::<Song>::new(),
            //TODO: error_handling
            cfg: RstyConfig::new().unwrap(),
            focus: None,
        }
    }

    pub fn refresh(self) {}

    pub fn set_focus(mut self, index: usize) {
        self.focus = Some(index);
    }
}

impl App for RstyJingle {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.cfg.debug {
            ctx.set_debug_on_hover(true);
        } else {
            ctx.set_debug_on_hover(false);
        }

        // let thing = DebugOptions {
        //     debug_on_hover: true,
        //     show_expand_width: true,
        //     show_expand_height: true,
        //     show_resize: true,
        // };

        // ctx.set_style(egui::style::Style {
        //     debug: thing,
        //     ..Default::default()
        // });

        if self.cfg.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.cfg.has_run == false {
            initgui(ctx, self);
        } else {
            if self.cfg.is_simple {
            } else {
                if self.songs.len() == 0 {
                    sdb_to_vec(&mut self.songs).expect("SDB_TO_VEC failed normal startup");
                }

                complex_layout(ctx, self);
            }
        }
    }
}

fn complex_layout(ctx: &egui::Context, rsty: &mut RstyJingle) {
    bottom_panel(ctx, rsty);

    side_panel(ctx, rsty);

    if rsty.cfg.settings_page {
        settings_page(ctx, rsty);
    } else {
        center_panel(ctx, rsty);
    }
}

fn trial(ctx: &egui::Context) {
    CentralPanel::default().show(ctx, |ui| {
        egui::Window::new("Kek")
            .resize(|r| r.fixed_size(egui::Vec2::new(300., 300.)))
            .show(ctx, |ui| {});
    });
}

fn initgui(ctx: &egui::Context, rsty: &mut RstyJingle) {
    CentralPanel::default().show(ctx, |ui| {
        egui::Window::new("First_time_conf")
            .resizable(false)
            .collapsible(false)
            .hscroll(false)
            .vscroll(false)
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("First time Settings");
                });

                ui.add_space(30.);

                egui::Grid::new("First_time_conf_table")
                    .num_columns(2)
                    .striped(true)
                    .spacing([0., 30.])
                    .show(ui, |ui| {
                        ui.label("Darkmode:");
                        let theme = ui.add(egui::Button::new(format!("{}", rsty.cfg.dark_mode)));
                        if theme.clicked() {
                            rsty.cfg.dark_mode = !rsty.cfg.dark_mode;
                        }
                        ui.end_row();

                        ui.label("Simple Layout:");
                        let layout = ui.add(egui::Button::new(format!("{}", rsty.cfg.is_simple)));
                        if layout.clicked() {
                            rsty.cfg.is_simple = !rsty.cfg.is_simple;
                        }
                        ui.end_row();

                        ui.label("Folders");
                        let add_folder_button = ui.add(egui::Button::new("Add Folder"));
                        if add_folder_button.clicked() {
                            rsty.cfg.folders.push(
                                FileDialog::new()
                                    .pick_folder()
                                    .expect("Uhf, you fucked up here buddy"),
                            );
                        }

                        ui.end_row();
                    });

                for folder in &rsty.cfg.folders {
                    ui.label(format!("{}", folder.as_path().display().to_string()));
                }

                ui.add_space(30.);
                ui.horizontal(|ui| {
                    ui.add_space(50.);
                    if ui.add(egui::Button::new("Submit")).clicked() {
                        rsty.cfg.has_run = true;

                        sql::sql_init().expect("init sql has failed");

                        parse_folder(&rsty.cfg.folders);

                        match fs::read_dir("/tmp") {
                            Ok(_) => rsty.cfg.is_linux = true,
                            Err(_) => rsty.cfg.is_linux = false,
                        }

                        rsty.cfg.save().expect("saving Rsty config has failed");

                        sdb_to_vec(&mut rsty.songs).expect("SDB_TO_VEC failed at startup");
                    }
                });
            });
    });
}

fn side_panel(ctx: &egui::Context, rsty: &mut RstyJingle) {
    SidePanel::left("Options")
        .resizable(false)
        .min_width(250.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.);
                let Settings = ui.add(egui::Button::new("Settings"));
                if Settings.clicked() {
                    rsty.cfg.settings_page = !rsty.cfg.settings_page
                }
            });

            ui.separator();
        });
}

fn bottom_panel(ctx: &egui::Context, rsty: &mut RstyJingle) {
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

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(15.0);
                ui.add(egui::ProgressBar::new(0.).desired_width(ui.available_width() / 2.));
                ui.add_space(10.0);
                match rsty.focus {
                    Some(x) => {
                        ui.label(&rsty.songs[rsty.focus.unwrap()].name);
                    }
                    None => {
                        ui.label("No song currently playing");
                    }
                }
                ui.add_space(8.0);

                ui.allocate_ui_with_layout(
                    egui::vec2(100., 100.),
                    Layout::bottom_up(Align::Center),
                    |ui| {
                        ui.horizontal(|ui| {
                            let previous = ui.add(egui::Button::new("⏮"));
                            let play = ui.add(egui::Button::new("▶"));
                            let next = ui.add(egui::Button::new("⏭"));

                            if previous.clicked() {
                                rsty.focus = Some(rsty.focus.unwrap() - 1)
                            }

                            if play.clicked() {
                                rsty.songs[rsty.focus.unwrap()].play_shitty();
                            }

                            if next.clicked() {
                                rsty.focus = Some(rsty.focus.unwrap() + 1)
                            }
                        });
                    },
                );
            });
        });
}

fn center_panel(ctx: &egui::Context, rsty: &mut RstyJingle) {
    CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("some_unique_id")
                .min_col_width(ui.available_width())
                .striped(true)
                .show(ui, |ui| {
                    for song in 0..rsty.songs.len() {
                        ui.vertical(|ui| {
                            ui.add_space(20.);
                            ui.horizontal(|ui| {
                                ui.add_space(5.);
                                if ui.link(&rsty.songs[song].name).clicked() {
                                    println!("current index: {:?}", rsty.focus);
                                    println!("current song: {}", &rsty.songs[song].name);
                                    println!("Song id: {}", &rsty.songs[song].id);

                                    if rsty.focus == Some(song) {
                                        rsty.focus = None
                                    } else {
                                        rsty.focus = Some(song);
                                    }

                                    println!("new index: {:?}", rsty.focus);
                                    println!("new song: {}", &rsty.songs[song].name);
                                }
                            });
                            ui.add_space(20.);
                        });

                        ui.end_row();
                    }
                });
            ui.allocate_space(Vec2 {
                x: ui.available_width(),
                y: 0.,
            });
        });
    });
}

fn settings_page(ctx: &egui::Context, rsty: &mut RstyJingle) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Grid::new("Options_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.vertical(|ui|{
                ui.add_space(20.);
                ui.horizontal(|ui|{
                ui.label("Debug:");
                if ui.button(format!("{}", rsty.cfg.debug)).clicked() {
                    rsty.cfg.debug = !rsty.cfg.debug;
                };
                ui.label("This Option allows you to toggle the Debug view, that can be helpful for developers");
                });
                ui.add_space(20.);
                });
                ui.end_row();
            });
    });
}
