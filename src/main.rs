use std::io::BufRead;
use std::io::Write;
use std::io::{self, IsTerminal};

use clap::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let stdin = io::stdin();
    let result = if stdin.is_terminal() {
        let args = nucleo_ui::cli::Cli::parse();
        nucleo_ui::interactive_fuzzy_find(None, Some(args))?
    } else {
        let stdin = stdin.lock();
        let mut lines = stdin.lines();
        let mut list = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            list.push(line);
        }
        nucleo_ui::interactive_fuzzy_find(Some(list), None)?
    };

    // Write output to stdout
    if let Some(result) = result {
        if let Err(err) = writeln!(io::stdout(), "{}", result) {
            return Err(err.into());
        };
    }

    Ok(())
}
