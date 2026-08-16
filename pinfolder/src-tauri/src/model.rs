use serde::{Deserialize, Serialize};

/// One entry in a project list — a folder directly under the chosen root
/// that has been initialized as a Pinfolder project (has a `.board-id`).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    /// Absolute path to the project folder.
    pub path: String,
    /// Unix epoch seconds, from the folder's mtime.
    pub modified_at: u64,
}

/// The full contents of a `board.json` file — one board's canvas.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoardDocument {
    pub version: u32,
    pub id: String,
    #[serde(default)]
    pub elements: Vec<serde_json::Value>,
    #[serde(default)]
    pub connectors: Vec<serde_json::Value>,
    #[serde(default)]
    pub frames: Vec<serde_json::Value>,
}

impl BoardDocument {
    pub fn new(id: String) -> Self {
        Self {
            version: 1,
            id,
            elements: Vec::new(),
            connectors: Vec::new(),
            frames: Vec::new(),
        }
    }
}

/// Persisted app-level settings — currently just the last folder the user
/// picked as their projects root, so re-opening the app skips the picker.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub last_root: Option<String>,
}
