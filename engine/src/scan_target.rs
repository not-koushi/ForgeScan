use std::path::Path;
use crate::fs_scan::collect_js_files;
use crate::obfuscation::analyze_file;
use crate::finding::Finding;

pub fn scan_path(path: &str) -> Vec<Finding> {
    let mut files = Vec::new();
    let mut findings = Vec::new();

    collect_js_files(Path::new(path), &mut files);

    for file in files {
        if let Some(finding) = analyze_file(&file) {
            findings.push(finding);
        }
    }

    findings
}