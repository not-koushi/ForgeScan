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

use crate::loader::load_package_json;
use crate::scanner::scan_dependencies;
use crate::report::print_warning;
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

    if cli.deps {
        println!("\n[INFO] Scanning package.json for typo-squatting...\n");

        let pkg = load_package_json("package.json");

        if let Some(dependencies) = pkg.dependencies {
            let dep_names: Vec<String> = dependencies.keys().cloned().collect();
            let findings = scan_dependencies(&dep_names);

            for finding in findings {
                print_warning(&finding);
            }
        }
    }
}