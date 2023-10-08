use std::{fs::File, io::Write, time::Instant, vec};

mod logger;
mod parse;

use clap::Parser as clapParse;
use rust_search::SearchBuilder;

use crate::{
    logger::{LogLevel, Logger},
    parse::ParseResult,
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

    /// The output location.
    #[arg(short, long)]
    output: String,
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
            "Indexing completed in {:?}, found {} file(s).",
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

    let generation_start: Instant = Instant::now();
    let j = serde_json::to_string(&parse_results);
    match j {
        Ok(json) => {
            let json_generator_result = json_generator(&args.output, json);
            let generation_duration = generation_start.elapsed();
            match json_generator_result {
                Ok(_) => logger.log(
                    LogLevel::INFO,
                    format!("Wrote to {} in {:?}", args.output, generation_duration),
                ),
                Err(e) => logger.log(LogLevel::ERROR, format!("Error writing to file: {}", e)),
            }
        }
        Err(err) => {
            logger.log(LogLevel::ERROR, err.to_string());
        }
    };
}

fn json_generator(output_location: &String, json: String) -> std::io::Result<()> {
    let mut file = File::create(output_location)?;
    file.write(json.as_bytes())?;

    Ok(())
}
