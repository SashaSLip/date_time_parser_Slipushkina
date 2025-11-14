/// Performed by Slipushkina Oleksandra

use std::env;
use std::fs;
use std::process;

use date_time_parser_Slipushkina::{parse_date, parse_time};

fn print_help() {
    println!("Date Time Parser CLI");
    println!("Commands:");
    println!("  parse <file>    - parse dates and times from a file");
    println!("  --help          - show this help");
    println!("  --credits       - show authors");
}

fn print_credits() {
    println!("Date Time Parser");
    println!("Author: Slipushkina Oleksandra");
    println!("Parses dates: YYYY-MM-DD and DD/MM/YYYY");
    println!("Parses times: 24h HH:MM and 12h HH:MM AM/PM");
}

fn parse_file(filename: &str) {
    let content = fs::read_to_string(filename);
    let content = match content {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to open file {}: {}", filename, e);
            process::exit(1);
        }
    };

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(d) = parse_date(line) {
            println!("Line {}: date {:04}-{:02}-{:02}", i + 1, d.year, d.month, d.day);
            continue;
        }

        if let Ok(t) = parse_time(line) {
            println!("Line {}: time {:02}:{:02}", i + 1, t.hour, t.minute);
            continue;
        }

        println!("Line {}: could not parse '{}'", i + 1, line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                println!("Please provide a file to parse");
                return;
            }
            parse_file(&args[2]);
        }
        "--help" => print_help(),
        "--credits" => print_credits(),
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}
