use config::CliParameters;
use domain::{lookup_acronym, KnownAcronym, TargetAcronym};
use fetcher::{ConfluenceFetcher, Fetcher, FileFetcher};
use output::{OutputFormat, OutputStyle};
use std::{sync::Arc, thread};

pub mod config;
mod domain;
mod fetcher;
mod output;

pub fn run(cli_parameters: &CliParameters) {
    let cli_acronym = &cli_parameters.acronym;
    let cli_format = match cli_parameters.format {
        Some(format) => format,
        None => OutputStyle::Cli,
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
        env_parameters
            .file_paths
            .unwrap()
            .iter()
            .for_each(|file_path| {
                fetchers.push(Arc::new(Box::new(FileFetcher::new(file_path.clone()))));
            });
    }

    let known_acronyms = fetchers
        .iter()
        .map(|fetcher| {
            let thread_fetcher = Arc::clone(fetcher);
            thread::spawn(move || thread_fetcher.fetch().ok())
        })
        .filter_map(|handle| handle.join().unwrap())
        .flatten()
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
