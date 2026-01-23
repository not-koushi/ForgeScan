use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "forgescan", about = "Supply-chain security scanner")]
pub struct Cli {
    /// Path to scan (e.g. src/, dist/)
    pub path: String,

    /// Include node_modules in scan
    #[arg(long)]
    pub include_deps: bool,

    /// Scan package.json for typo-squatting
    #[arg(long)]
    pub deps: bool,

    /// Emit JSON output
    #[arg(long)]
    pub json: bool,
}