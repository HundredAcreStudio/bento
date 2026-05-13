//! Bento shell library entry point.
//!
//! Issue #2 wires up the scaffolding only. The real REPL lands with issue #3.

/// Run the shell. Currently a placeholder that handles `--version` and prints
/// a "not yet implemented" stub for any other invocation.
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("bento {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    eprintln!(
        "bento {}: REPL not yet implemented (see issue #3)",
        env!("CARGO_PKG_VERSION")
    );
}
