use crate::domain::{self, KnownAcronym};
use scraper::{Html, Selector};
use std::error::Error;

pub trait Fetcher {
    fn fetch(&self) -> Result<Vec<domain::KnownAcronym>, Box<dyn Error>>;
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
    fn fetch(&self) -> Result<Vec<domain::KnownAcronym>, Box<dyn Error>> {
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
        let parsed_html = Html::parse_fragment(text_content);
        let selector = Selector::parse("p").unwrap();
        let known_acronyms = parsed_html
            .select(&selector)
            .filter_map(|element| {
                let text = element.text().collect::<String>();
                let hyphens = [" – ", " - ", " — "];
                let splits: Vec<Vec<&str>> = hyphens
                    .iter()
                    .filter_map(|hyphen| {
                        if !text.contains(hyphen) {
                            return None;
                        }
                        let parts = text.splitn(2, hyphen).collect::<Vec<&str>>();
                        return Some(parts);
                    })
                    .collect();
                if splits.len() == 1 {
                    let parts = splits.first().unwrap();
                    Some(domain::KnownAcronym::new(
                        parts[0].to_string(),
                        parts[1].to_string(),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<KnownAcronym>>();

        return Ok(known_acronyms);
    }
}
