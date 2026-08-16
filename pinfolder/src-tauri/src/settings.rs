use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::fsutil::write_atomic;
use crate::model::AppSettings;

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("could not resolve app config dir: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("could not create app config dir: {e}"))?;
    Ok(dir.join("settings.json"))
}

pub fn load(app: &AppHandle) -> Result<AppSettings, String> {
    let path = settings_path(app)?;
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("could not read settings: {e}"))?;
    serde_json::from_str(&raw).or_else(|_| Ok(AppSettings::default()))
}

pub fn save(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let raw = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("could not serialize settings: {e}"))?;
    write_atomic(&path, &raw)
}
