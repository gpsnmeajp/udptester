#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{FontData, FontDefinitions, ScrollArea};
use std::{
    env,
    net::UdpSocket,
    sync::mpsc::{self, Receiver},
};

fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || loop {
        let socket = UdpSocket::bind("127.0.0.1:12345").unwrap();
        loop {
            let mut buf: [u8; 65535] = [0; 65535];

            match socket.recv(&mut buf) {
                Ok(received) => {
                    let cnv: String =
                        String::from_utf8(buf[..received].to_vec()).unwrap_or("?(UTF-8 Decode error)".to_owned());
                    println!("received {received} bytes {:?}", cnv);
                    tx.send(cnv).unwrap();
                }
                Err(e) => println!("recv function failed: {e:?}"),
            }
        }
    });

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
        Box::new(|cc| Box::new(MyApp::new(cc, rx))),
    )
}

struct MyApp {
    rx: Receiver<String>,
    send_socket: UdpSocket,
    ip_input: String,
    port_input: String,
    send_input: String,
    received_data: Vec<String>,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, rx: Receiver<String>) -> Self {
        Self {
            rx: rx,
            send_socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            ip_input: "127.0.0.1".to_owned(),
            port_input: "12345".to_owned(),
            send_input: String::new(), //default value
            received_data: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint(); //毎フレーム更新を要求

        for r in self.rx.try_iter() {
            self.received_data.push(r);
        }

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

        // ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UDP Tester");
            ui.horizontal(|ui| {
                ui.label("IP:");
                ui.text_edit_singleline(&mut self.ip_input);
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.port_input);
            });

            ui.horizontal(|ui| {
                ui.label("Send:");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("SEND").clicked() {
                        println!("{}", self.send_input);
                        self.send_socket
                            .send_to(
                                self.send_input.as_bytes(),
                                self.ip_input.as_str().to_owned() + ":" + self.port_input.as_str(),
                            )
                            .unwrap();
                        self.send_input = "".to_owned();
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
            let rows = self.received_data.len();
            ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, rows, |ui, row_range| {
                    for row in row_range {
                        ui.label(self.received_data[row].to_owned());
                    }
                });
        });
    }
}
