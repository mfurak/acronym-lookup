mod domain;
mod fetcher;
mod output;

pub fn run(target_acronym: &str) {
    let target_acronym = domain::TargetAcronym::new(target_acronym);

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
    if let Some(results) = res {
        let output_format = output::OutputFormat {
            numbering: false,
            format: output::OutputStyle::CLI,
        };

        output_format.print_output(&results);
    }
}
