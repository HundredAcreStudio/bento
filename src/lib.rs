//! Bento shell library entry point.
//!
//! The REPL spine: read a line, echo it back, loop until EOF. Parsing,
//! built-ins, and signal handling land in later milestone issues.

use std::io::{self, BufRead, Write};

/// Run the shell. Handles `--version` / `-V` and otherwise enters the REPL.
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("bento {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if let Err(e) = repl() {
        eprintln!("bento: {e}");
    }
}

fn repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut line = String::new();

    loop {
        write!(stdout, "bento> ")?;
        stdout.flush()?;

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            // EOF (Ctrl-D on an empty line). Move to a fresh line so the
            // caller's prompt doesn't collide with ours.
            writeln!(stdout)?;
            return Ok(());
        }
        stdout.write_all(line.as_bytes())?;
        if !line.ends_with('\n') {
            writeln!(stdout)?;
        }
        stdout.flush()?;
    }
}
