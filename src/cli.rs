use std::path::PathBuf;

use clap::{arg, command, Parser};

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub directory: bool,

    #[arg(short, long, required = false)]
    pub path: Option<PathBuf>,

    #[arg(long, required = false)]
    pub min_depth: Option<usize>,

    #[arg(long, required = false)]
    pub max_depth: Option<usize>,
}
