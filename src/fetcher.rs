use crate::domain::KnownAcronym;
use std::{fs, io};

const SEPARATORS: [&str; 4] = [" – ", " - ", " — ", " : "];

pub trait Fetcher: Send + Sync {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, Error>;
}

pub struct Error {}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self {}
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Self {}
    }
}

struct ConfluenceFetcherConfig {
    user_name: String,
    api_token: String,
    base_url: String,
    page_id: String,
}

pub struct Confluence {
    config: ConfluenceFetcherConfig,
}

impl Confluence {
    pub const fn new(
        user_name: String,
        api_token: String,
        base_url: String,
        page_id: String,
    ) -> Self {
        Self {
            config: ConfluenceFetcherConfig {
                user_name,
                api_token,
                base_url,
                page_id,
            },
        }
    }
}

impl Fetcher for Confluence {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, Error> {
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

        Ok(known_acronyms)
    }
}

struct FileFetcherConfig {
    file_path: String,
}

pub struct File {
    config: FileFetcherConfig,
}

impl File {
    pub const fn new(file_path: String) -> Self {
        Self {
            config: FileFetcherConfig { file_path },
        }
    }
}

impl Fetcher for File {
    fn fetch(&self) -> Result<Vec<KnownAcronym>, Error> {
        let file_content = fs::read_to_string(&self.config.file_path)?;
        Ok(file_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(parse_acronym)
            .collect::<Vec<KnownAcronym>>())
    }
}

fn parse_acronym(line: &str) -> Option<KnownAcronym> {
    let splits: Vec<Vec<&str>> = SEPARATORS
        .iter()
        .filter_map(|separator| {
            if line.contains(separator) {
                let parts = line.splitn(2, separator).collect::<Vec<&str>>();
                return Some(parts);
            }
            None
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
