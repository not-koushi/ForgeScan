mod entropy;
mod levenshtein;
mod package_scan;
mod report;
mod package_json;
mod loader;
mod scanner;
mod finding;
mod fs_scan;
mod obfuscation;
mod scan_target;
mod cli;

use cli::Cli;
use serde_json::json;
use std::path::{Path, PathBuf};

use clap::Parser;
use crate::loader::load_package_json;
use crate::scanner::scan_dependencies;
use crate::scan_target::scan_path;
use crate::finding::{Finding, Severity};

fn main() {
    let cli = Cli::parse();

    println!("ForgeScan scanning path: {}\n", cli.path);

    // Central findings collection
    let mut all_findings: Vec<Finding> = Vec::new();

    // Path-based obfuscation scan
    let path_findings = scan_path(&cli.path);

    for finding in path_findings {
        if matches!(finding.severity, Severity::Low) {
            continue;
        }

        if !cli.json {
            println!(
                "[{:?}] {} (entropy: {:.2})",
                finding.severity,
                finding.path.as_deref().unwrap_or("unknown"),
                finding.entropy.unwrap_or(0.0)
            );
        }

        all_findings.push(finding);
    }

    // Dependency typo-squatting scan
    if cli.deps {
        if !cli.json {
            println!("\n[INFO] Scanning package.json for typo-squatting...\n");
        }

        let scan_path = PathBuf::from(&cli.path);
        let project_root = scan_path.parent().unwrap_or(Path::new("."));
        let pkg_path = project_root.join("package.json");

        match load_package_json(pkg_path.to_str().unwrap()) {
            Ok(pkg) => {
                if let Some(dependencies) = pkg.dependencies {
                    let dep_names: Vec<String> =
                        dependencies.keys().cloned().collect();

                    let dep_findings = scan_dependencies(&dep_names);

                    for finding in dep_findings {
                        if !cli.json {
                            println!(
                                "[MEDIUM] Package \"{}\" resembles \"{}\"",
                                finding.package.as_deref().unwrap_or("unknown"),
                                finding.similar_to.as_deref().unwrap_or("unknown")
                            );
                        }

                        all_findings.push(finding);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "[ERROR] Failed to load package.json at {}: {}",
                    pkg_path.display(),
                    e
                );
            }
        }
    }

    // JSON output (CI/Machine Consumption)
    if cli.json {
        let output = json!({
            "tool": "forgescan",
            "version": "1.0.0",
            "findings": all_findings
        });

        println!(
            "{}",
            serde_json::to_string_pretty(&output)
                .expect("Failed to serialize ForgeScan output")
        );
    }
}
