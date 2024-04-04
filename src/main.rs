use std::io::{self, IsTerminal};
use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        nucleo_ui::launch_ui(None)?;
    } else {
        let stdin = stdin.lock();
        let mut lines = stdin.lines();
        let mut list = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            list.push(line);
        }
        nucleo_ui::launch_ui(Some(list))?;
    }
    Ok(())
}
