use crate::levenshtein::levenshtein;
use crate::finding::{Finding, Severity};

const POPULAR_PACKAGES: [&str; 5] = [
    "express",
    "react",
    "lodash",
    "axios",
    "chalk",
];

pub fn scan_dependency(name: &str) -> Option<Finding> {
    for popular in POPULAR_PACKAGES {
        let distance = levenshtein(name, popular);

        if distance > 0 && distance <= 2 {
            return Some(Finding {
                category: "typosquatting".into(),
                severity: Severity::Medium,
                path: None,
                entropy: None,
                package: Some(name.to_string()),
                similar_to: Some(popular.to_string()),
            });
        }
    }
    None
}