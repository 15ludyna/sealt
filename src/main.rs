mod printer;
mod cli;
mod utils;

use cli::{Cli};
use utils::{json};

use std::time::{Instant};
use structopt::StructOpt;

#[macro_use]
extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // start timer
    let start = Instant::now();

    // make request
    println!("\n[~]Making request for {}\n", args.domain);
    let res = reqwest::get(&args.domain)
        .await
        .unwrap()
        .status();

    // calculate duration
    let duration = start.elapsed();

    // print results
    printer::print_success_msg("Success! Results:");
    println!(
        "· url = {}\n· status = {}\n· duration = {}ms\n", 
        args.domain, res, duration.as_millis()
    );

    // Format output in json
    if args.json {
        println!("[~]Format results to json");
        json::format_to_json(
            args.domain, 
            res.to_string(), 
            duration.as_millis()
        );
        printer::print_success_msg("Success!\n");
    }

    printer::print_thanks_msg("Thank's for choose us!\n");

    Ok(())
}
