use anyhow::Result;
use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use enigo::{
    Enigo,
    Settings,
    Mouse,
    Keyboard,
    Direction,
    Axis,
    Coordinate,
};

use crate::model::{MacroEvent, MacroEventKind};
use crate::keymap::{parse_mouse_button, parse_enigo_key};

pub fn play_events(
    events: Vec<MacroEvent>,
    speed: f64,
    stop: Arc<AtomicBool>,
) -> Result<()> {

    // 🔴 Windows'ta Enigo artık Result döndürür
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings)
        .expect("Failed to initialize Enigo");

    for (i, ev) in events.into_iter().enumerate() {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        // 🔥 İLK EVENT ASLA BEKLEMEZ
        let delay = if i == 0 {
            0
        } else {
            ((ev.dt_ms as f64) / speed).max(0.0) as u64
        };

        if delay > 0 {
            thread::sleep(Duration::from_millis(delay));
        }

        match ev.kind {
            MacroEventKind::MouseMove { x, y } => {
                let _ = enigo.move_mouse(x as i32, y as i32, Coordinate::Abs);
            }

            MacroEventKind::MouseDown { button } => {
                let _ = enigo.button(parse_mouse_button(&button), Direction::Press);
            }

            MacroEventKind::MouseUp { button } => {
                let _ = enigo.button(parse_mouse_button(&button), Direction::Release);
            }

            MacroEventKind::MouseWheel { delta_x, delta_y } => {
                if delta_y != 0.0 {
                    let _ = enigo.scroll(delta_y as i32, Axis::Vertical);
                }
                if delta_x != 0.0 {
                    let _ = enigo.scroll(delta_x as i32, Axis::Horizontal);
                }
            }

            MacroEventKind::KeyDown { key } => {
                if let Some(k) = parse_enigo_key(&key) {
                    let _ = enigo.key(k, Direction::Press);
                }
            }

            MacroEventKind::KeyUp { key } => {
                if let Some(k) = parse_enigo_key(&key) {
                    let _ = enigo.key(k, Direction::Release);
                }
            }
        }
    }

    Ok(())
}
