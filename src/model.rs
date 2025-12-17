use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroFile {
    pub version: u32,
    pub events: Vec<MacroEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroEvent {
    pub dt_ms: u64,
    pub kind: MacroEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MacroEventKind {
    MouseMove { x: f64, y: f64 },
    MouseDown { button: String },
    MouseUp { button: String },
    MouseWheel { delta_x: f64, delta_y: f64 },

    KeyDown { key: String },
    KeyUp { key: String },
}
