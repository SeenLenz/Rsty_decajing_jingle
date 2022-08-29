use std::fs;

use crate::gui::RstyJingle;
use eframe::{egui, run_native, NativeOptions};

mod audio;
mod gui;
mod opint;
mod sql;

fn main() {
    match fs::read("./config/RstyConfig_cfg.json") {
        Ok(f_) => {}
        Err(f_) => match fs::read("./database/rsty_jingle.db") {
            Ok(_) => gui::RstyConfig {
                has_run: true,
                ..Default::default()
            }
            .save()
            .unwrap(),
            Err(_) => gui::RstyConfig {
                has_run: false,
                ..Default::default()
            }
            .save()
            .unwrap(),
        },
    }

    match fs::read("./database/rsty_jingle.db") {
        Ok(_) => gui::RstyConfig {
            has_run: true,
            ..Default::default()
        }
        .save()
        .unwrap(),
        Err(_) => gui::RstyConfig {
            has_run: false,
            ..Default::default()
        }
        .save()
        .unwrap(),
    };

    let win_options: NativeOptions = NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 960.0)),
        min_window_size: Some(egui::vec2(960.0, 960.0)),
        ..Default::default()
    };
    run_native(
        "rsty_jingle",
        win_options,
        Box::new(|cc| Box::new(RstyJingle::new())),
    );
}
