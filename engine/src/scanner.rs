use crate::package_scan::scan_dependency;
use crate::finding::Finding;

pub fn scan_dependencies(deps: &Vec<String>) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();

    for dep in deps {
        if let Some(finding) = scan_dependency(dep) {
            findings.push(finding);
        }
    }

    findings
}