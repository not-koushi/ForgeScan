mod entropy;
mod levenshtein;
mod package_scan;
mod report;

use crate::package_scan::scan_dependency;
use crate::report::generate_report;

fn main() {
    println!("ForgeScan initialized.");

    let result = scan_dependency("expres");

    if let Some(alert) = result {
        print_warning(&alert);
    }
}