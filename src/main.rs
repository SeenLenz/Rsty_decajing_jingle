// use std::io;
// use std::mem::drop;
// use std::io::Write;
// use std::fs::ReadDir;

// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, Sink};

use eframe::{egui::CentralPanel, App, run_native, NativeOptions};


mod file_handling;
mod audio_handling;

fn main(){

    let app: rsty_jingle  = rsty_jingle;
    let win_options = NativeOptions::default();

    run_native("rsty_jingle", win_options, Box::new(|cc| Box::new(rsty_jingle::new(cc))));

}

#[derive(Default)]
struct rsty_jingle;

impl rsty_jingle {
    
    fn new(cc: &eframe::CreationContext<'_>) -> Self{

        Self::default()

    }
}

impl App for rsty_jingle {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{
            ui.label("I dont give a shit");
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }


}