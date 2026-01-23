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
mod node_modules_scan;

use loader::load_package_json;
use scanner::scan_dependencies;
use report::print_warning;
use node_modules_scan::scan_node_modules;
use crate::finding::Severity;

fn main() {
    let pkg = load_package_json("../samples/package.json");

    if let Some(dependencies) = pkg.dependencies {
        let dep_names: Vec<String> = dependencies.keys().cloned().collect();
        let findings = scan_dependencies(&dep_names);

        for finding in findings {
            print_warning(&finding);
        }
    }

    println!("\nScanning node_modules for obfuscated malware...\n");

    let malware_findings = scan_node_modules("../node_modules");

    for finding in malware_findings {
        if matches!(finding.severity, Severity::Low){
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