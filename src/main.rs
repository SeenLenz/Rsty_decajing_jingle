// use std::io;
// use std::mem::drop;
// use std::io::Write;
// use std::fs::ReadDir;

// use std::fs::File;
// use std::io::BufReader;
// use rodio::{Decoder, OutputStream, Sink};

use eframe::{
    egui::CentralPanel, 
    App, 
    run_native, 
    NativeOptions,
    egui::{TopBottomPanel, SidePanel, Frame, Context, style::Visuals, ScrollArea}
};
use egui::ProgressBar;


mod file_handling;
mod audio_handling;


fn main(){

    
    let app: rsty_jingle  = rsty_jingle;
    let win_options = NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 960.0)),
        ..Default::default()
    };

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
        let mut fekete_pako = Visuals {dark_mode: true, ..Default::default()};
        egui::Context::set_visuals(ctx, fekete_pako);

        TopBottomPanel::bottom("navbar").min_height(100.0).show(ctx, |ui| {
            ui.label("Player");
            ui.add(egui::ProgressBar::new(0.0))
        });

        SidePanel::right("Options").resizable(false).min_width(200.0).show(ctx,|ui|{
            ui.label("Options/search");
            ui.add(egui::Button::new("🌙")).clicked()
        });

        CentralPanel::default().show(ctx, |ui|{

            ScrollArea::vertical().auto_shrink([false,false]).show(ui, |ui|{

                for i in 1..100 {
                    ui.label("Songs.....................................................................................................................................................................................................................................................................");
                }
            });
        });
    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}
}

