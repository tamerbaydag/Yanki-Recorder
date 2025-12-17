use tray_icon::{TrayIcon, TrayIconBuilder, Icon};


pub struct Tray {
    tray: TrayIcon,
    icon_normal: Icon,
    icon_rec: Icon,
    icon_play: Icon,
}

fn load_icon(bytes: &[u8]) -> Icon {
    let img = image::load_from_memory(bytes)
        .expect("Invalid PNG")
        .into_rgba8();

    let (w, h) = img.dimensions();
    Icon::from_rgba(img.into_raw(), w, h)
        .expect("Failed to create tray icon")
}

pub fn init_tray() -> Tray {
    let icon_normal = load_icon(include_bytes!("../assets/tray.png"));
    let icon_rec = load_icon(include_bytes!("../assets/tray_rec.png"));
    let icon_play = load_icon(include_bytes!("../assets/tray_play.png"));

    let tray = TrayIconBuilder::new()
        .with_icon(icon_normal.clone())
        .with_tooltip("Yankı – Macro Recorder")
        .build()
        .expect("Failed to build tray");

    Tray {
        tray,
        icon_normal,
        icon_rec,
        icon_play,
    }
}

impl Tray {
    pub fn set_normal(&self) {
        let _ = self.tray.set_icon(Some(self.icon_normal.clone()));
    }

    pub fn set_rec(&self) {
        let _ = self.tray.set_icon(Some(self.icon_rec.clone()));
    }

    pub fn set_play(&self) {
        let _ = self.tray.set_icon(Some(self.icon_play.clone()));
    }
}
