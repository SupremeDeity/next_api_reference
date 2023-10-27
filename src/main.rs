use std::{
    fs::{self},
    path::Path,
    time::Instant,
};

mod generators;
mod logger;
mod parse;

use clap::Parser as clapParse;
use rust_search::SearchBuilder;

use crate::{
    logger::{LogLevel, Logger},
    parse::{ParseResult, Parser},
};

#[derive(clapParse, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose logging.
    #[arg(short, long)]
    verbose: bool,

    /// Location to find route handlers from.
    #[arg(short, long, default_value_t = String::from("./"))]
    location: String,

    /// The directory to output to.
    #[arg(short, long)]
    output: String,

    // Only use the json generator
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let args = Cli::parse();

    // --- Logger INIT ---
    let max_level = if args.verbose {
        LogLevel::VERBOSE
    } else {
        LogLevel::INFO
    };
    let logger: Logger = Logger::new(max_level);
    // Enable ANSI support for Windows 10 & above.
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();
    // --- Logger INIT end ---

    println!();
    logger.log(LogLevel::INFO, format!("Indexing {}", args.location));

    let indexing_start = Instant::now();
    let search: Vec<String> = SearchBuilder::default()
        .location(&args.location)
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
            "Indexing completed in {:?}, found {} file(s).",
            indexing_duration,
            { search.len() }
        ),
    );

    logger.log(LogLevel::INFO, "Parsing...");
    let parsing_start: Instant = Instant::now();
    let mut parse_results: Vec<ParseResult> = vec![];
    let parser = Parser::new(args.location.to_owned());
    for path in &search {
        let parse_result: ParseResult = parser.parse(path);
        parse_results.push(parse_result);
        logger.log(LogLevel::VERBOSE, format!("Parsed {}", path))
    }

    let parsing_duration = parsing_start.elapsed();

    logger.log(
        LogLevel::INFO,
        format!("Parsing completed in {:?}", parsing_duration),
    );

    let output_path = Path::new(&args.output);

    if let Err(e) = fs::create_dir_all(&output_path) {
        logger.log(LogLevel::ERROR, e.to_string());
    }

    if args.json {
        let json_generation_start: Instant = Instant::now();
        match generators::json_generator(output_path, parse_results) {
            Ok(_) => {
                let generation_duration = json_generation_start.elapsed();
                logger.log(
                    LogLevel::INFO,
                    format!(
                        "Completed generating JSON at '{}' in {:?}",
                        output_path.display(),
                        generation_duration
                    ),
                )
            }
            Err(e) => logger.log(
                LogLevel::ERROR,
                format!(
                    "Error generating JSON at '{}': {}",
                    output_path.display(),
                    e
                ),
            ),
        };

        return;
    }

    let site_generation_start: Instant = Instant::now();
    match generators::html_generator(output_path, parse_results) {
        Ok(_) => {
            let generation_duration = site_generation_start.elapsed();
            logger.log(
                LogLevel::INFO,
                format!(
                    "Completed generating site at '{}' in {:?}",
                    output_path.display(),
                    generation_duration
                ),
            )
        }
        Err(e) => logger.log(
            LogLevel::ERROR,
            format!(
                "Error generating site at '{}': {}",
                output_path.display(),
                e
            ),
        ),
    };
}
