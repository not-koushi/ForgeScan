use crate::package_scan::scan_dependency;

pub fn scan_dependencies(deps: &Vec<String>) -> Vec<String> {
    let mut findings = Vec::new();

    for dep in deps {
        if let Some(alert) = scan_dependency(dep) {
            findings.push(alert);
        }
    }

    findings
}