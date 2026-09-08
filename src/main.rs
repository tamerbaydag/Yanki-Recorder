#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Receiver,
    Arc,
};

mod model;
mod io;
mod keymap;
mod recorder;
mod player;
mod tray;

use model::{MacroEvent, MacroFile};

fn load_window_icon() -> egui::IconData {
    let bytes = include_bytes!("../assets/tray.png"); // istersen app.png yap
    let img = image::load_from_memory(bytes)
        .expect("Invalid PNG for window icon")
        .into_rgba8();

    let (w, h) = img.dimensions();
    egui::IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 360.0])
            .with_min_inner_size([300.0, 300.0])
            .with_icon(load_window_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Yankı – Recorder",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

struct App {
    events: Vec<MacroEvent>,

    speed: f64,
    sample_ms: u64,
    status: String,

    rx: Option<Receiver<MacroEvent>>,
    ctrl_rx: Option<Receiver<recorder::ControlMsg>>,
    recorder_running: bool,
    recording_flag: Arc<AtomicBool>,

    stop_play: Arc<AtomicBool>,
    show_about: bool,
    tray: Option<tray::Tray>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            events: vec![],
            speed: 1.0,
            sample_ms: 15,
            status: "Ready".into(),
            show_about: false,
            rx: None,
            ctrl_rx: None,
            recorder_running: false,
            recording_flag: Arc::new(AtomicBool::new(false)),

            stop_play: Arc::new(AtomicBool::new(true)), // 🔴 true = duruyor

            tray: None,
        }
    }
}

impl App {
    fn ensure_recorder(&mut self) {
        if self.recorder_running {
            return;
        }

        let (tx_ev, rx_ev) = std::sync::mpsc::channel();
        let (tx_ctrl, rx_ctrl) = std::sync::mpsc::channel();

        recorder::start_record_stream(
            tx_ev,
            tx_ctrl,
            self.recording_flag.clone(),
            self.sample_ms,
        );

        self.rx = Some(rx_ev);
        self.ctrl_rx = Some(rx_ctrl);
        self.recorder_running = true;
    }

    fn drain_events(&mut self) {
        if let Some(rx) = &self.rx {
            while let Ok(ev) = rx.try_recv() {
                self.events.push(ev);
            }
        }
    }

    fn drain_ctrl(&mut self, ctx: &egui::Context) {
        let mut toggle_rec = false;
        let mut toggle_play = false;

        if let Some(rx) = &self.ctrl_rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    recorder::ControlMsg::ToggleRecordHotkey => toggle_rec = true,
                    recorder::ControlMsg::PlayHotkey => toggle_play = true,
                }
            }
        }

        if toggle_rec {
            self.toggle_record(ctx);
        }
        if toggle_play {
            self.toggle_play(ctx);
        }
    }

    fn toggle_record(&mut self, ctx: &egui::Context) {
        let on = self.recording_flag.load(Ordering::Relaxed);

        if !on {
            self.ensure_recorder();
            self.recording_flag.store(true, Ordering::Relaxed);
            self.status = "Recording… (Ctrl+Q)".into();

            if let Some(tray) = &self.tray {
                tray.set_rec();
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        } else {
            self.recording_flag.store(false, Ordering::Relaxed);
            self.status = "Recording stopped".into();

            if let Some(tray) = &self.tray {
                tray.set_normal();
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
    }

    fn toggle_play(&mut self, ctx: &egui::Context) {
        // 🔴 Eğer oynuyorsa → DUR
        if !self.stop_play.load(Ordering::Relaxed) {
            self.stop_play.store(true, Ordering::Relaxed);
            self.status = "Playback stopped".into();

            if let Some(tray) = &self.tray {
                tray.set_normal();
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            return;
        }

        if self.events.is_empty() {
            self.status = "No events to play".into();
            return;
        }

        // ▶️ BAŞLAT (sonsuz)
        self.stop_play.store(false, Ordering::Relaxed);
        self.status = "Playing… (F9 to stop)".into();

        if let Some(tray) = &self.tray {
            tray.set_play();
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));

        let events = self.events.clone();
        let speed = self.speed;
        let stop = self.stop_play.clone();

        std::thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                let _ = player::play_events(events.clone(), speed, stop.clone());
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.tray.is_none() {
            self.tray = Some(tray::init_tray());
        }

        self.ensure_recorder();
        self.drain_events();
        self.drain_ctrl(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(" Yankı – Recorder ");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Record (Ctrl+Q)").clicked() {
                    self.toggle_record(ctx);
                }
                if ui.button("Play / Stop (F9)").clicked() {
                    self.toggle_play(ctx);
                }

            });
            if ui.button("About").clicked() {
                self.show_about = true;
            }

            ui.add(egui::Slider::new(&mut self.speed, 0.2..=3.0).text("Speed"));
            ui.add(egui::Slider::new(&mut self.sample_ms, 1..=50).text("Mouse sample ms"));

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("macro", &["json"])
                        .save_file()
                    {
                        let mf = MacroFile {
                            version: 1,
                            events: self.events.clone(),
                        };
                        let _ = io::save_json(path.to_string_lossy().as_ref(), &mf);
                        self.status = "Saved".into();
                    }
                }

                if ui.button("Load").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("macro", &["json"])
                        .pick_file()
                    {
                        if let Ok(mf) = io::load_json(path.to_string_lossy().as_ref()) {
                            self.events = mf.events;
                            self.status = "Loaded".into();
                        }
                    }
                }

                if ui.button("Clear").clicked() {
                    self.events.clear();
                }
            });

            ui.separator();
            ui.label(format!("Events: {}", self.events.len()));
            ui.label(format!("Status: {}", self.status));
        });
        if self.show_about {
            egui::Window::new("About Yankı")
                .open(&mut self.show_about)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.heading("Yankı –  Recorder");
                    ui.label("Version: 1.0.0");
                    ui.separator();

                    ui.label(
                        "Yankı is a lightweight macro recorder designed to capture\n\
                 and replay keyboard and mouse interactions on Windows."
                    );

                    ui.separator();
                    ui.label("Designed & Developed by:");
                    ui.label("Tamer Baydağ – Systems Engineer");

                    ui.separator();
                    ui.label("Built with:");
                    ui.label("• Rust");
                    ui.label("• egui / eframe");
                    ui.label("• rdev / enigo");

                    ui.separator();
                    ui.small("© 2025 Tamer Baydağ. All rights reserved.");
                });
        }


        ctx.request_repaint();
    }
}
