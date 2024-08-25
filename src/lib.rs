use config::CliParameters;
use domain::{lookup_acronym, KnownAcronym, TargetAcronym};
use fetcher::{ConfluenceFetcher, Fetcher, FileFetcher};
use output::{OutputFormat, OutputStyle};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

pub mod config;
mod domain;
mod fetcher;
mod output;

pub fn run(cli_parameters: &CliParameters) {
    let cli_acronym = &cli_parameters.acronym;
    let cli_format = match cli_parameters.format {
        Some(format) => format,
        None => OutputStyle::CLI,
    };

    let target_acronym = TargetAcronym::new(cli_acronym);

    let mut fetchers: Vec<Arc<Box<dyn Fetcher + Sync>>> =
        vec![Arc::new(Box::new(ConfluenceFetcher::new(
            std::env::var("AL_CONFLUENCE_USER_NAME").unwrap(),
            std::env::var("AL_CONFLUENCE_API_TOKEN").unwrap(),
            std::env::var("AL_CONFLUENCE_BASE_URL").unwrap(),
            std::env::var("AL_CONFLUENCE_PAGE_ID").unwrap(),
        )))];

    if std::env::var("AL_FILE_PATHS").is_ok() {
        fetchers.push(Arc::new(Box::new(FileFetcher::new(
            std::env::var("AL_FILE_PATHS").unwrap(),
        ))));
    }

    let handles = fetchers
        .iter()
        .map(|fetcher| {
            let thread_fetcher = Arc::clone(fetcher);
            let handle = thread::spawn(move || thread_fetcher.fetch().ok());
            handle
        })
        .collect::<Vec<JoinHandle<Option<Vec<KnownAcronym>>>>>();

    let known_acronyms = handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap().unwrap())
        .collect::<Vec<KnownAcronym>>();

    let res = lookup_acronym(&target_acronym, known_acronyms);
    match res {
        Some(results) => {
            let output_format = OutputFormat { format: cli_format };

            output_format.print_output(&results, &target_acronym);
        }
        None => eprintln!("404 - No acronyms found. :("),
    }
}
