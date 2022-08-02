use eframe::{run_native, NativeOptions, egui};
use crate::gui::RstyJingle;
use std::fs;

mod gui;
mod audio;
mod opint;
mod sql;

fn main() {

    let win_options: NativeOptions = NativeOptions {
    initial_window_size: Some(egui::vec2(960.0, 960.0)), ..Default::default()};
    run_native("rsty_jingle", win_options, Box::new(|cc| Box::new(RstyJingle::new())));

}