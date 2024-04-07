mod domain;
mod fetcher;
mod output;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    acronym: String,
    #[arg(short, long, value_enum)]
    format: Option<output::OutputStyle>,
}

pub fn run(config: &Cli) {
    let cli_acronym = config.acronym.clone();
    let cli_format = match config.format {
        Some(format) => format,
        None => output::OutputStyle::CLI,
    };

    let target_acronym = domain::TargetAcronym::new(&cli_acronym);

    let fetchers: Vec<Box<dyn fetcher::Fetcher>> = vec![Box::new(fetcher::ConfluenceFetcher::new(
        std::env::var("CONFLUENCE_USER_NAME").unwrap(),
        std::env::var("CONFLUENCE_API_TOKEN").unwrap(),
        std::env::var("CONFLUENCE_BASE_URL").unwrap(),
        std::env::var("CONFLUENCE_PAGE_ID").unwrap(),
    ))];

    let known_acronyms = fetchers
        .iter()
        .filter_map(|fetcher| fetcher.fetch().ok())
        .flatten()
        .collect();

    let res = domain::lookup_acronym(&target_acronym, known_acronyms);
    match res {
        Some(results) => {
            let output_format = output::OutputFormat {
                numbering: false,
                format: cli_format,
            };

            output_format.print_output(&results);
        }
        None => println!("404 - No acronyms found. :("),
    }
}
