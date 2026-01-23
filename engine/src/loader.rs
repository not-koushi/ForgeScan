use std::fs;
use std::error::Error;
use crate::package_json::PackageJson;

pub fn load_package_json(path: &str) -> Result<PackageJson, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let pkg = serde_json::from_str(&content)?;
    Ok(pkg)
}