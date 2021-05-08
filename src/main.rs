use std::io;
use std::io::{BufRead, Write};

use errors::term_errors::Error;

mod errors;

fn main() -> Result<(), Error>{
    crossterm::terminal::enable_raw_mode()?;

    let input = io::stdin();
    let output = io::stdout();

    loop {
        let mut handle_out = output.lock();
        write!(handle_out, "Type a command >")?;
        handle_out.flush()?;

        let line = input.lock().lines().next().unwrap()?;
        if line == "quit" {
            break;
        }

        write!(handle_out, "You typed '{}'\n\r", line)?;
        handle_out.flush()?;
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
