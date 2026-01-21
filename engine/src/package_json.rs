use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub dependencies: Option<HashMap<String, String>>,
}