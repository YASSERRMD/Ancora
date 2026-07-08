use std::env;

/// Entry point that dispatches CLI sub-commands.
pub fn run() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or("graph.yaml");
            let store_kind = args.get(3).map(|s| s.as_str()).unwrap_or("memory");
            if let Err(e) = crate::spec::run_graph(path, store_kind) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Some("serve") => {
            let port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(7700);
            if let Err(e) = crate::studio::serve(port) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Some("help") | None => print_help(),
        Some(cmd) => {
            eprintln!("unknown command: {cmd}");
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("ancora - reference agent runtime");
    println!();
    println!("USAGE:");
    println!("  ancora run <graph.yaml> [store]");
    println!("  ancora serve [port]");
    println!();
    println!("STORE:");
    println!("  memory    in-memory (default)");
    println!("  sqlite    SQLite (creates ancora.db)");
}
