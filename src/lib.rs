use domain::{lookup_acronym, TargetAcronym};
use fetcher::{ConfluenceFetcher, Fetcher};
use output::{OutputFormat, OutputStyle};
use std::env;

mod domain;
mod fetcher;
mod output;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    acronym: String,
    #[arg(short, long, value_enum)]
    format: Option<OutputStyle>,
}

pub fn run(config: &Cli) {
    let cli_acronym = config.acronym.clone();
    let cli_format = match config.format {
        Some(format) => format,
        None => OutputStyle::CLI,
    };

    let target_acronym = TargetAcronym::new(&cli_acronym);

    let fetchers: Vec<Box<dyn Fetcher>> = vec![Box::new(ConfluenceFetcher::new(
        env::var("CONFLUENCE_USER_NAME").unwrap(),
        env::var("CONFLUENCE_API_TOKEN").unwrap(),
        env::var("CONFLUENCE_BASE_URL").unwrap(),
        env::var("CONFLUENCE_PAGE_ID").unwrap(),
    ))];

    let known_acronyms = fetchers
        .iter()
        .filter_map(|fetcher| fetcher.fetch().ok())
        .flatten()
        .collect();

    let res = lookup_acronym(&target_acronym, known_acronyms);
    match res {
        Some(results) => {
            let output_format = OutputFormat {
                numbering: false,
                format: cli_format,
            };

            output_format.print_output(&results, &target_acronym);
        }
        None => println!("404 - No acronyms found. :("),
    }
}
