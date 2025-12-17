use crate::model::MacroFile;
use anyhow::Result;
use std::fs;

pub fn save_json(path: &str, mf: &MacroFile) -> Result<()> {
    let s = serde_json::to_string_pretty(mf)?;
    fs::write(path, s)?;
    Ok(())
}

pub fn load_json(path: &str) -> Result<MacroFile> {
    let s = fs::read_to_string(path)?;
    let mf: MacroFile = serde_json::from_str(&s)?;
    Ok(mf)
}
