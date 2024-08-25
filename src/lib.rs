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

    let env_parameters = config::EnvParameters::load();

    let target_acronym = TargetAcronym::new(cli_acronym);

    let mut fetchers: Vec<Arc<Box<dyn Fetcher + Sync>>> =
        vec![Arc::new(Box::new(ConfluenceFetcher::new(
            env_parameters.confluence.user_name.clone(),
            env_parameters.confluence.api_token.clone(),
            env_parameters.confluence.base_url.clone(),
            env_parameters.confluence.page_id.clone(),
        )))];

    if env_parameters.file_paths.is_some() {
        fetchers.push(Arc::new(Box::new(FileFetcher::new(
            env_parameters.file_paths.unwrap(),
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
