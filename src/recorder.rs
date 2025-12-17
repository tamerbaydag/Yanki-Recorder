use rdev::{listen, Event, EventType, Key};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
    Mutex,
};
use std::time::Instant;

use crate::model::{MacroEvent, MacroEventKind};

#[derive(Debug, Clone, Copy)]
pub enum ControlMsg {
    ToggleRecordHotkey, // Ctrl+Q
    PlayHotkey,         // F9
    
}

pub fn start_record_stream(
    event_tx: Sender<MacroEvent>,
    ctrl_tx: Sender<ControlMsg>,
    recording_flag: Arc<AtomicBool>,
    sample_ms: u64,
) {
    std::thread::spawn(move || {
        let last_mouse = Mutex::new(Instant::now());
        let last_any = Mutex::new(Instant::now());

        let _ = listen(move |e: Event| {
            // === HOTKEYLER (GLOBAL) ===
            match e.event_type {
                EventType::KeyPress(Key::KeyQ) => {
                    // Ctrl+Q algılaması (rdev modifier vermez → sade Q)
                    let _ = ctrl_tx.send(ControlMsg::ToggleRecordHotkey);
                    return;
                }
                EventType::KeyPress(Key::F9) => {
                    let _ = ctrl_tx.send(ControlMsg::PlayHotkey);
                    return;
                }
                _ => {}
            }

            // === KAYIT AÇIK DEĞİLSE ÇIK ===
            if !recording_flag.load(Ordering::Relaxed) {
                return;
            }

            let now = Instant::now();
            let mut last = last_any.lock().unwrap();
            let dt_ms = now.duration_since(*last).as_millis() as u64;
            *last = now;

            let kind = match e.event_type {
                EventType::MouseMove { x, y } => {
                    let mut lm = last_mouse.lock().unwrap();
                    if lm.elapsed().as_millis() < sample_ms as u128 {
                        return;
                    }
                    *lm = Instant::now();

                    Some(MacroEventKind::MouseMove {
                        x: x as f64,
                        y: y as f64,
                    })
                }

                EventType::ButtonPress(b) => Some(MacroEventKind::MouseDown {
                    button: format!("{:?}", b),
                }),

                EventType::ButtonRelease(b) => Some(MacroEventKind::MouseUp {
                    button: format!("{:?}", b),
                }),

                EventType::Wheel { delta_x, delta_y } => Some(
                    MacroEventKind::MouseWheel {
                        delta_x: delta_x as f64,
                        delta_y: delta_y as f64,
                    }
                ),

                EventType::KeyPress(k) => Some(MacroEventKind::KeyDown {
                    key: format!("{:?}", k),
                }),

                EventType::KeyRelease(k) => Some(MacroEventKind::KeyUp {
                    key: format!("{:?}", k),
                }),

                
            };

            if let Some(kind) = kind {
                let _ = event_tx.send(MacroEvent { dt_ms, kind });
            }
        });
    });
}
