// ============================================
// DEMO 1: CLI Tool — minigrep
// ============================================
// Zeigt: Argument-Parsing, Datei-I/O, Iteratoren
// Echte Projekte: ripgrep (rg), bat, fd, exa, delta
//
// Ausführen: rustc cli.rs && ./cli "fn" cli.rs

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <query> <file>", args[0]);
        process::exit(1);
    }

    let query    = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Fehler beim Lesen von '{}': {}", filename, err);
        process::exit(1);
    });

    println!("Suche '{}' in '{}':\n", query, filename);

    let results = search(query, &contents);

    if results.is_empty() {
        println!("  (keine Treffer)");
    } else {
        for (line_num, line) in results {
            println!("  {:>4}: {}", line_num, line);
        }
    }
}

// Zero-Copy: Rückgabe sind Slices in den Originaldaten — keine Allokation
fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}
