// use std::collections::VecDeque;
// use std::fmt::format;
// use std::io;
// use std::mem::drop;
// use std::io::Write;
// use std::fs::ReadDir;

// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, Sink};
// use std::path::PathBuf;

// const IMAGE_SIZE: Vec2 = Vec2::new(100.,100.);
#![allow(unused)]

use std::fmt::format;

use egui_extras::image::RetainedImage;
use eframe::{
    egui,
    run_native,
    NativeOptions,
    App, 
    egui::{TopBottomPanel, 
        SidePanel, 
        style::Visuals, 
        ScrollArea, 
        Layout, 
        Resize, 
        CentralPanel, 
        LayerId, 
        Ui,
        Align}, epaint::tessellator::path
};
use music_player::gui;

mod lib;



fn main(){
    let app = gui::RstyJingle::new();
    let win_options = NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 960.0)),
        ..Default::default()
    };

    run_native("rsty_jingle", win_options, Box::new(|cc| Box::new(RstyJingle::new())));
}

const PADDING: f32 = 50.;
pub struct SongCard {
    id: i32,
    img: RetainedImage,
    path: String,
    name: String,
    duration: String,
    date_added: String,
    clicks: i32,
    favourite: i32,
}

#[derive(Default)]

pub struct RstyConfig{
    dark_mode: bool,
    main_page: bool,
}

pub struct RstyJingle{
    songs: Vec<SongCard>,
    cfg: RstyConfig,
}

impl RstyConfig {
    fn default() -> Self{
        RstyConfig{
            dark_mode: true,
            main_page: true,
        }
    }
}

// RetainedImage::from_image_bytes("1.png",include_bytes!("1.png")).unwrap()
impl RstyJingle {

    pub fn new() -> RstyJingle{

        let iter =(0..20).map(|a| SongCard{
            id: a,
            img: RetainedImage::from_image_bytes("1.png",include_bytes!("1.png")).unwrap(),
            path: "./path/to_the/song".to_string(),
            name: format!("name of song {}",a),
            duration: format!("{}:{}{}",a,a,a),
            date_added: format!("2022.7.{}",a),
            clicks: 0,
            favourite: 0,
        });

        RstyJingle {
            songs:Vec::from_iter(iter),
            cfg: RstyConfig::default()
        }
    }
}


impl App for RstyJingle {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.cfg.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        };

        render_main(self, ctx);
    
    fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    }

        


    fn render_main(main: &mut RstyJingle,ctx: &egui::Context){
        
        TopBottomPanel::bottom("navbar")
        .min_height(100.0)
        .max_height(150.0)
        .show(ctx, |ui| {

            ui.label("Player");
            ui.with_layout(Layout::bottom_up(Align::Center), |ui|{
                ui.add_space(15.0);
                ui.add(egui::ProgressBar::new(100.0).desired_width(ui.available_width() / 2.))
            });
        });

        SidePanel::left("Options")
        .resizable(false)
        .min_width(200.0)
        .show(ctx,|ui|{

            ui.label("Options/search");
            let theme_changer = ui.add(egui::Button::new("🌙"));
            let page_changer = ui.add(egui::Button::new("🎵"));
            if theme_changer.clicked(){
                main.cfg.dark_mode = !main.cfg.dark_mode;
            }
            if page_changer.clicked(){
                main.cfg.main_page = !main.cfg.main_page;
            }

        });

        let song = RetainedImage::from_image_bytes("1.png",include_bytes!("1.png")).unwrap();

        if main.cfg.main_page {

            CentralPanel::default().show(ctx, |ui|{

                ScrollArea::vertical().auto_shrink([false,false]).show(ui, |ui|{
                    egui::Grid::new("somadse_unique_id")
                    .num_columns(1)
                    .spacing([40.0, 4.0])
                    .min_col_width(ui.available_width())
                    .min_row_height(80.)
                    .striped(true)
                    .show(ui, |ui|{
                        for i in 1..20{
                                ui.horizontal(|ui|{
                                ui.label("Thing");
                                ui.label("Thing");
                                ui.label("Thing")
                            });
                            ui.end_row();
                        };
                        });
                    });
                });

        } else {

            CentralPanel::default().show(ctx, |ui|{

                ui.label("something else");
                
            });  
        };
    }

    fn render_SongCard(ui: &mut Ui, song: &SongCard){
        song.img.show(ui);
        ui.label("Thing");
        ui.end_row();
    }

    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}
}

// ScrollArea::vertical().auto_shrink([false,false]).show(ui, |ui|{

//     egui::Grid::new("some_unique_id")
//     .num_columns(1)
//     .spacing([40.0, 4.0])
//     .striped(true)
//     .show(ui, |ui|{

//         ui.label("this one is the first row aka row number one aka 1");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 2");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 3");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 4");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 5");
//         ui.end_row();

//     });

//     egui::Window::new("My Window").resizable(true)
//     .default_width(280.0).show(ctx, |ui| {
//         egui::Grid::new("somadse_unique_id")
//     .num_columns(1)
//     .spacing([40.0, 4.0])
//     .striped(true)
//     .show(ui, |ui|{

//         ui.label("this one is the first row aka row number one aka 1");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 2");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 3");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 4");
//         ui.end_row();

//         ui.label("this one is the first row aka row number one aka 5");
//         ui.end_row();

//     });
//     });


// });