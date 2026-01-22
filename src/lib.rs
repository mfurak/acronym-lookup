use domain::{KnownAcronym, TargetAcronym, lookup_acronym};
use std::{sync::Arc, thread};

pub mod config;
mod domain;
mod fetcher;
mod output;

/// # Panics
pub fn run(cli_parameters: &config::CliParameters) {
    let cli_acronym = &cli_parameters.acronym;
    let cli_format = cli_parameters
        .format
        .map_or(output::Style::Cli, |format| format);

    let env_parameters = config::EnvParameters::load();

    let target_acronym = TargetAcronym::new(cli_acronym);

    let mut fetchers: Vec<Arc<Box<dyn fetcher::Fetcher + Sync>>> =
        vec![Arc::new(Box::new(fetcher::Confluence::new(
            env_parameters.confluence.user_name.clone(),
            env_parameters.confluence.api_token.clone(),
            env_parameters.confluence.base_url.clone(),
            env_parameters.confluence.page_id.clone(),
        )))];

    if let Some(parameter_paths) = env_parameters.file_paths {
        for file_path in parameter_paths.iter() {
            fetchers.push(Arc::new(Box::new(fetcher::File::new(file_path.clone()))));
        }
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

    let res = lookup_acronym(&target_acronym, &known_acronyms);
    match res {
        Some(results) => {
            let output_format = output::Format { format: cli_format };

            output_format.print_output(&results, &target_acronym);
        }
        None => eprintln!("404 - No acronyms found. :("),
    }
}
