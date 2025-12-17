use enigo::{Button as EButton, Key as EKey};




pub fn parse_mouse_button(s: &str) -> EButton {
    match s {
        "Left" => EButton::Left,
        "Right" => EButton::Right,
        "Middle" => EButton::Middle,
        _ => EButton::Left,
    }
}

/// Rdev Key string -> Enigo Key (temel mapping)
pub fn parse_enigo_key(s: &str) -> Option<EKey> {
    // Özel tuşlar
    let k = match s {
        "Return" | "Enter" => EKey::Return,
        "Tab" => EKey::Tab,
        "Backspace" => EKey::Backspace,
        "Escape" => EKey::Escape,
        "Space" => EKey::Space,

        "UpArrow" => EKey::UpArrow,
        "DownArrow" => EKey::DownArrow,
        "LeftArrow" => EKey::LeftArrow,
        "RightArrow" => EKey::RightArrow,

        "ShiftLeft" | "ShiftRight" => EKey::Shift,
        "ControlLeft" | "ControlRight" => EKey::Control,
        "Alt" | "AltGr" => EKey::Alt,
        "MetaLeft" | "MetaRight" | "Super" => EKey::Meta,

        "F1" => EKey::F1,
        "F2" => EKey::F2,
        "F3" => EKey::F3,
        "F4" => EKey::F4,
        "F5" => EKey::F5,
        "F6" => EKey::F6,
        "F7" => EKey::F7,
        "F8" => EKey::F8,
        "F9" => EKey::F9,
        "F10" => EKey::F10,
        "F11" => EKey::F11,
        "F12" => EKey::F12,
        _ => {
            // Harf/rakam (KeyA, KeyB... Num1... vs.)
            if let Some(ch) = extract_single_alnum(s) {
                return Some(EKey::Unicode(ch));
            }
            return None;
        }
    };

    Some(k)
}

// "KeyA" -> 'a', "Num1" -> '1'
fn extract_single_alnum(s: &str) -> Option<char> {
    if let Some(rest) = s.strip_prefix("Key") {
        if rest.len() == 1 {
            return rest.chars().next().map(|c| c.to_ascii_lowercase());
        }
    }
    if let Some(rest) = s.strip_prefix("Num") {
        if rest.len() == 1 {
            return rest.chars().next();
        }
    }
    None
}
