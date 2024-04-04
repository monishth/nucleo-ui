use std::path::PathBuf;

use clap::{arg, command, Parser};

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, required = false)]
    pub directory: Option<PathBuf>,

    #[arg(long, required = false)]
    pub min_depth: Option<usize>,

    #[arg(long, required = false)]
    pub max_depth: Option<usize>,
}
