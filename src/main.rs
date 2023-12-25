mod printer;
mod cli;
mod utils;

use cli::{Cli};
use std::time::{Instant};
use structopt::StructOpt;

#[macro_use]
extern crate reqwest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::from_args();

    // validate domain
    match utils::domain::validate_domain(&args.domain) {
        true => {
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
                utils::json::format_to_json(
                    args.domain, 
                    res.to_string(), 
                    duration.as_millis()
                );
                printer::print_success_msg("Success!\n");
            }

            printer::print_thanks_msg("Thank's for choose us!\n");

        }
        false => {
            let msg = format!("Invalide domain: {}", args.domain);
            printer::print_error_msg(&msg);
        }
    }
    Ok(())
}
