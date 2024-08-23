use crate::domain::KnownAcronym;
use std::{fs, io};

const HYPHENS: [&str; 3] = [" – ", " - ", " — "];

pub trait Fetcher: Send + Sync {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, FetcherError>;
}

pub struct FetcherError {}

impl From<reqwest::Error> for FetcherError {
    fn from(_: reqwest::Error) -> Self {
        FetcherError {}
    }
}

impl From<io::Error> for FetcherError {
    fn from(_: io::Error) -> Self {
        FetcherError {}
    }
}

struct ConfluenceFetcherConfig {
    user_name: String,
    api_token: String,
    base_url: String,
    page_id: String,
}

pub struct ConfluenceFetcher {
    config: ConfluenceFetcherConfig,
}

impl ConfluenceFetcher {
    pub fn new(user_name: String, api_token: String, base_url: String, page_id: String) -> Self {
        ConfluenceFetcher {
            config: ConfluenceFetcherConfig {
                user_name,
                api_token,
                base_url,
                page_id,
            },
        }
    }
}

impl Fetcher for ConfluenceFetcher {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, FetcherError> {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "{}/wiki/api/v2/pages/{}?body-format=view",
            self.config.base_url, self.config.page_id
        );
        let response = client
            .get(&url)
            .basic_auth(&self.config.user_name, Some(&self.config.api_token))
            .send()?;
        let json_body = response.json::<serde_json::Value>()?;
        let text_content = json_body["body"]["view"]["value"].as_str().unwrap();
        let parsed_html = scraper::Html::parse_fragment(text_content);
        let selector = scraper::Selector::parse("p").unwrap();
        let known_acronyms = parsed_html
            .select(&selector)
            .map(|element| element.text().collect::<String>())
            .filter_map(|text| parse_acronym(&text))
            .collect::<Vec<KnownAcronym>>();

        return Ok(known_acronyms);
    }
}

struct FileFetcherConfig {
    file_paths: Vec<String>,
}

pub struct FileFetcher {
    config: FileFetcherConfig,
}

impl FileFetcher {
    pub fn new(file_paths: String) -> Self {
        FileFetcher {
            config: FileFetcherConfig {
                file_paths: file_paths
                    .split(",")
                    .filter_map(|path| {
                        if path.is_empty() {
                            None
                        } else {
                            Some(path.to_owned())
                        }
                    })
                    .collect(),
            },
        }
    }
}

impl Fetcher for FileFetcher {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, FetcherError> {
        let results = self
            .config
            .file_paths
            .iter()
            .filter_map(|file_path| {
                if let Ok(file_content) = fs::read_to_string(&file_path) {
                    Some(
                        file_content
                            .lines()
                            .filter(|line| !line.is_empty())
                            .filter_map(parse_acronym)
                            .collect::<Vec<KnownAcronym>>(),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<KnownAcronym>>();
        Ok(results)
    }
}

fn parse_acronym(line: &str) -> Option<KnownAcronym> {
    let splits: Vec<Vec<&str>> = HYPHENS
        .iter()
        .filter_map(|hyphen| {
            if !line.contains(hyphen) {
                return None;
            }
            let parts = line.splitn(2, hyphen).collect::<Vec<&str>>();
            return Some(parts);
        })
        .collect();
    // splits will only have a single value depending on which hyphen it matched
    if splits.len() == 1 {
        let parts = splits.first().unwrap();
        Some(KnownAcronym::new(parts[0], parts[1]))
    } else {
        None
    }
}
