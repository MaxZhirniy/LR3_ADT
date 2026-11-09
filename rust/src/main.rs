mod array;
mod commands;
mod baseball;
mod pattern_matcher;
mod huffman;
mod genealogy;
mod set_operations;
mod doubly_list;
mod forward_list;
mod hash_table;
mod lru;
mod stack;
mod storage;

use std::env;
use std::io::{self, Write};
use std::path::Path;
use storage::Database;

fn run_once(db: &mut Database, path: &Path, query: &str) -> Result<(), String> {
    let (output, modified) = commands::run(db, query)?;
    if modified { storage::save(path, db)?; }
    if !output.is_empty() { println!("{output}"); }
    Ok(())
}
fn execute() -> Result<(), String> {
    let mut path = String::from("file.data");
    let mut query: Option<String> = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" | "-file" => path = args.next().ok_or("missing --file value")?,
            "--query" | "-query" => query = Some(args.next().ok_or("missing --query value")?),
            "--help" | "-h" => {
                println!("dbms-rust [--file file.data] [--query 'BASEBALL 5 2 C D +']");
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }
    let path = Path::new(&path);
    let mut db = storage::load(path)?;
    if let Some(q) = query { return run_once(&mut db, path, &q); }
    println!("DBMS (Rust). Type HELP for commands, EXIT to finish.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut line = String::new();
        if stdin.read_line(&mut line).map_err(|e| e.to_string())? == 0 { break; }
        if line.trim().eq_ignore_ascii_case("EXIT") { break; }
        if let Err(e) = run_once(&mut db, path, &line) { eprintln!("ERROR: {e}"); }
    }
    Ok(())
}
fn main() {
    if let Err(e) = execute() { eprintln!("ERROR: {e}"); std::process::exit(1); }
}

