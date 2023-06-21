#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{emath::RectTransform, FontData, FontDefinitions, Rounding, ScrollArea};
use std::env;

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        centered: true,
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "UDP Tester",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    send_input: String,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            send_input: String::new(), //default value
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.request_repaint(); //毎フレーム更新を要求

        /*
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\yumindb.ttf")),
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
        */

        // ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UDP Tester");
            ui.horizontal(|ui| {
                ui.label("Send:");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("SEND").clicked() {
                        println!("{}", self.send_input);
                    }
                    //ui.visuals_mut().extreme_bg_color = egui::Color32::RED;
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.send_input),
                    );
                });
            });

            ui.add_space(4.0);

            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            let rows = 100;
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, rows, |ui, row_range| {
                    for row in row_range {
                        ui.label(format!("{:?}", row));
                    }
                });
        });
    }
}
