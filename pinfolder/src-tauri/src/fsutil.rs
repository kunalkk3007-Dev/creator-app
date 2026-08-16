use std::fs;
use std::path::Path;

/// Writes `contents` to `path` without ever leaving a half-written file behind:
/// write to a sibling temp file, flush it to disk, then rename it over the
/// real path. A rename within the same directory is atomic on every platform
/// Tauri targets, so a crash or power loss mid-save can't corrupt board.json.
pub fn write_atomic(path: &Path, contents: &str) -> Result<(), String> {
    let dir = path
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(dir).map_err(|e| format!("could not create {}: {e}", dir.display()))?;

    let tmp_path = dir.join(format!(
        ".{}.tmp",
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "write".to_string())
    ));

    {
        use std::io::Write;
        let mut file = fs::File::create(&tmp_path)
            .map_err(|e| format!("could not create {}: {e}", tmp_path.display()))?;
        file.write_all(contents.as_bytes())
            .map_err(|e| format!("could not write {}: {e}", tmp_path.display()))?;
        file.sync_all()
            .map_err(|e| format!("could not flush {}: {e}", tmp_path.display()))?;
    }

    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("could not replace {}: {e}", path.display())
    })
}
