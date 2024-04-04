use std::io::BufRead;
use std::io::{self, IsTerminal};

use clap::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    if stdin.is_terminal() {
        let args = nucleo_ui::cli::Cli::parse();
        nucleo_ui::launch_ui(None, Some(args))?;
    } else {
        let stdin = stdin.lock();
        let mut lines = stdin.lines();
        let mut list = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            list.push(line);
        }
        nucleo_ui::launch_ui(Some(list), None)?;
    }
    Ok(())
}
