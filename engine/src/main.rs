mod entropy;
mod levenshtein;
mod package_scan;
mod report;
mod package_json;
mod loader;
mod scanner;

use loader::load_package_json;
use scanner::scan_dependencies;
use report::print_warning;

fn main() {
    let pkg = load_package_json("../samples/package.json");

    if let Some(dependencies) = pkg.dependencies {
        let dep_names = dependencies.keys().cloned().collect();
        let findings = scan_dependencies(&dep_names);

        for finding in findings {
            print_warning(&finding);
        }
    }
}