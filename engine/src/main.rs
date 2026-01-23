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

use clap::Parser;
use cli::Cli;
use scan_target::scan_path;
use finding::Severity;

fn main() {
    let cli = Cli::parse();

    println!("ForgeScan scanning path: {}\n", cli.path);

    let findings = scan_path(&cli.path);

    for finding in findings {
        if matches!(finding.severity, Severity::Low) {
            continue;
        }

        println!(
            "[{:?}] {} (entropy: {:.2})",
            finding.severity,
            finding.file,
            finding.entropy
        );
    }

    if cli.include_deps {
        println!("\n[INFO] Dependency scanning enabled (node_modules)");
        let dep_findings = scan_path("node_modules");

        for finding in dep_findings {
            if matches!(finding.severity, Severity::Low) {
                continue;
            }

            println!(
                "[{:?}] {} (entropy: {:.2})",
                finding.severity,
                finding.file,
                finding.entropy
            );
        }
    }
}