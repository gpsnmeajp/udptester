#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{emath::RectTransform, FontData, FontDefinitions, Rounding};
use std::env;

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        centered: true,
        default_theme: eframe::Theme::Light,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native("UDP Tester", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            //default value
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.request_repaint(); //毎フレーム更新を要求

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\yumin.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);

        // ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
        });
    }
}
