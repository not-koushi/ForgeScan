use crate::levenshtein::levenshtein;

const POPULAR_PACKAGES: [&str; 5] = [
    "express",
    "react",
    "lodash",
    "axios",
    "chalk",
];

pub fn scan_dependency(name: &str) -> Option<String> {
    for popular in POPULAR_PACKAGES {
        let distance = levenshtein(name, popular);

        if distance > 0 && distance <= 2 {
            return Some(format!(
                "Possible typo-squatting detected: '{}' resembles '{}'",
                name, popular
            ));
        }
    }
    None
}