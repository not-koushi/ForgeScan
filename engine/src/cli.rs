use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "forgescan")]
#[command(about = "Supply-chain security scanner for npm projects")]
pub struct Cli {
    /// Path to scan (e.g. src/, dist/)
    pub path: String,

    /// Include node_modules in scan
    #[arg(long)]
    pub include_deps: bool,
}