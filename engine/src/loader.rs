use std::fs;
use crate::package_json::PackageJson;

pub fn load_package_json(path: &str) -> PackageJson {
    let content = fs::read_to_string(path)
        .expect("Failed to read package.json");
    
    serde_json::from_str(&content)
        .expect("Invalid package.json format")
}