use std::fs;
use std::path::Path;

pub fn collect_js_files(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                collect_js_files(&path, files);
            } else if let Some(ext) = path.extension() {
                if ext == "js" {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
}