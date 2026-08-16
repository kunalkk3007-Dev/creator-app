use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::fsutil::write_atomic;
use crate::model::{BoardDocument, ProjectSummary};
use crate::settings;

const BOARD_ID_FILE: &str = ".board-id";
const BOARD_FILE: &str = "board.json";

/// Opens the native "choose a folder" dialog and, if the user picks one,
/// remembers it as the app's projects root for next launch.
#[tauri::command]
pub async fn pick_root_folder(app: AppHandle) -> Result<Option<String>, String> {
    let picked = app.dialog().file().blocking_pick_folder();
    let Some(file_path) = picked else {
        return Ok(None);
    };
    let path = file_path
        .into_path()
        .map_err(|e| format!("invalid folder selection: {e}"))?;
    let path_str = path.to_string_lossy().to_string();

    let mut current = settings::load(&app)?;
    current.last_root = Some(path_str.clone());
    settings::save(&app, &current)?;

    Ok(Some(path_str))
}

/// Returns the previously chosen root, if any, so the app can skip the
/// folder picker on subsequent launches.
#[tauri::command]
pub fn get_last_root(app: AppHandle) -> Result<Option<String>, String> {
    Ok(settings::load(&app)?.last_root)
}

fn folder_modified_at(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn read_board_id(project_dir: &Path) -> Option<String> {
    fs::read_to_string(project_dir.join(BOARD_ID_FILE))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Lists every immediate subfolder of `root` that has been initialized as a
/// Pinfolder project (i.e. carries a `.board-id`). Folders the user keeps in
/// the same root for other reasons are silently skipped.
#[tauri::command]
pub fn list_projects(root: String) -> Result<Vec<ProjectSummary>, String> {
    let root_path = PathBuf::from(&root);
    let entries = fs::read_dir(&root_path).map_err(|e| format!("could not read {root}: {e}"))?;

    let mut projects = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(id) = read_board_id(&path) else {
            continue;
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        projects.push(ProjectSummary {
            id,
            name,
            path: path.to_string_lossy().to_string(),
            modified_at: folder_modified_at(&path),
        });
    }

    projects.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(projects)
}

/// Validates a user-supplied project name is safe to use as a single path
/// segment — no separators, no traversal, not empty once trimmed.
fn validate_project_name(name: &str) -> Result<&str, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Project name can't be empty.".to_string());
    }
    if trimmed == "." || trimmed == ".." {
        return Err("That name isn't allowed.".to_string());
    }
    if trimmed.contains(['/', '\\']) {
        return Err("Project name can't contain a slash.".to_string());
    }
    Ok(trimmed)
}

/// Creates a new project: a subfolder of `root` holding an empty
/// `board.json` and a stable `.board-id` that survives future folder renames.
#[tauri::command]
pub fn create_project(root: String, name: String) -> Result<ProjectSummary, String> {
    let name = validate_project_name(&name)?;
    let project_dir = PathBuf::from(&root).join(name);

    if project_dir.exists() {
        return Err(format!("\"{name}\" already exists in this folder."));
    }
    fs::create_dir_all(&project_dir)
        .map_err(|e| format!("could not create {}: {e}", project_dir.display()))?;

    let id = Uuid::new_v4().to_string();
    write_atomic(&project_dir.join(BOARD_ID_FILE), &id)?;

    let board = BoardDocument::new(id.clone());
    let board_json = serde_json::to_string_pretty(&board)
        .map_err(|e| format!("could not serialize new board: {e}"))?;
    write_atomic(&project_dir.join(BOARD_FILE), &board_json)?;

    Ok(ProjectSummary {
        id,
        name: name.to_string(),
        path: project_dir.to_string_lossy().to_string(),
        modified_at: folder_modified_at(&project_dir),
    })
}

/// Reads and parses `board.json` at `path` (the project folder, not the
/// file itself). Missing optional fields are filled with their defaults by
/// serde rather than failing the whole board open.
#[tauri::command]
pub fn read_board(path: String) -> Result<BoardDocument, String> {
    let board_path = PathBuf::from(&path).join(BOARD_FILE);
    let raw = fs::read_to_string(&board_path)
        .map_err(|e| format!("could not read {}: {e}", board_path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("board.json is corrupted: {e}"))
}

/// Atomically overwrites `board.json` at `path` (the project folder) with
/// `doc`. Safe to call frequently — see `fsutil::write_atomic`.
#[tauri::command]
pub fn write_board(path: String, doc: BoardDocument) -> Result<(), String> {
    let board_path = PathBuf::from(&path).join(BOARD_FILE);
    let raw = serde_json::to_string_pretty(&doc)
        .map_err(|e| format!("could not serialize board: {e}"))?;
    write_atomic(&board_path, &raw)
}
