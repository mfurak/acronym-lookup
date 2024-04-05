mod domain;
mod fetcher;

enum OutputFormat {
    CLI,
    TEXT,
    JSON,
}

struct ResultFormat {
    numbering: bool,
    format: OutputFormat,
}

pub fn run() {
    let target_acronym = domain::TargetAcronym::new("ap".to_string());

    let fetchers: Vec<Box<dyn fetcher::Fetcher>> = vec![Box::new(fetcher::ConfluenceFetcher::new(
        "username".to_string(),
        "api_token".to_string(),
        "base_url".to_string(),
        "page_id".to_string(),
    ))];
    let known_acronyms: Vec<domain::KnownAcronym> = fetchers
        .iter()
        .filter_map(|fetcher| fetcher.fetch().ok())
        .flatten()
        .collect();

    let res = domain::lookup_acronym(&target_acronym, known_acronyms);
    println!("{:?}", res);
}
