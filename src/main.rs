use std::{time::Instant, vec};

mod logger;
mod parse;

use clap::Parser as clapParse;
use rust_search::SearchBuilder;
use swc_common::comments::Comments;

use crate::{
    logger::{LogLevel, Logger},
    parse::ParseResult,
};

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

    // --- Logger INIT ---
    let max_level = if args.verbose {
        LogLevel::VERBOSE
    } else {
        LogLevel::INFO
    };
    let logger: Logger = Logger::new(max_level);
    // --- Logger INIT end ---

    println!();
    logger.log(LogLevel::INFO, format!("Indexing {}", args.location));

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
            logger.log(LogLevel::VERBOSE, format!("Found path: {path}"));
        }
    }

    logger.log(
        LogLevel::INFO,
        format!(
            "Indexing completed in {:?}, found {} files.",
            indexing_duration,
            { search.len() }
        ),
    );

    logger.log(LogLevel::INFO, "Parsing...");
    let parsing_start: Instant = Instant::now();
    let mut parse_results: Vec<ParseResult> = vec![];
    for path in &search {
        let parse_result: ParseResult = parse::parse(path);
        parse_results.push(parse_result);
        logger.log(LogLevel::VERBOSE, format!("Parsed {}", path))
    }

    let parsing_duration = parsing_start.elapsed();

    logger.log(
        LogLevel::INFO,
        format!("Parsing completed in {:?}", parsing_duration),
    );

    for result in parse_results {
        logger.log(
            LogLevel::INFO,
            format!("{}: {:?}", result.path, result.method_metadata),
        )
    }
}
