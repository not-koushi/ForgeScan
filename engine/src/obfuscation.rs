use std::fs;
use crate::entropy::shannon_entropy;
use crate::finding::{Finding, Severity};

pub fn analyze_file(path: &str) -> Option<Finding> {
    let content = fs::read_to_string(path).ok()?;
    let entropy = shannon_entropy(&content);

    let severity = if entropy > 5.5 {
        Severity::High
    } else if entropy > 4.8 {
        Severity::Medium
    } else {
        Severity::Low
    };

    Some(Finding {
        category: "obfuscation".into(),
        severity,
        path: Some(path.to_string()),
        entropy: Some(entropy),
        package: None,
        similar_to: None,
    })
}