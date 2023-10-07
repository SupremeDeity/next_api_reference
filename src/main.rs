use std::time::Instant;

mod parse;

use clap::Parser as clapParse;
use rust_search::SearchBuilder;

#[derive(clapParse, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Location to run program.
    #[arg(short, long, default_value_t = String::from("./"))]
    location: String,
}

fn main() {
    let args = Args::parse();

    println!("Indexing {}", args.location);
    let indexing_start = Instant::now();
    let search: Vec<String> = SearchBuilder::default()
        .location(args.location)
        .search_input("route")
        .ext("(js|ts)")
        .strict()
        .build()
        .collect();

    let indexing_duration = indexing_start.elapsed();

    if args.verbose {
        for path in &search {
            println!("Found path: {path}")
        }
    }

    println!(
        "Indexing completed in {:?}, found {} files.",
        indexing_duration,
        { search.len() }
    );

    println!("Parsing...");
    let parsing_start: Instant = Instant::now();
    for path in &search {
        parse::parse(path);
        if args.verbose {
            println!("Parsed {}", path);
        }
    }

    let parsing_duration = parsing_start.elapsed();

    println!("Parsing completed in {:?}", parsing_duration);
}
