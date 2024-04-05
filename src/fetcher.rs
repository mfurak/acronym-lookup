use crate::domain;
use std::error::Error;

pub trait Fetcher {
    fn fetch(&self) -> Result<Vec<domain::KnownAcronym>, Box<dyn Error>>;
}

pub struct ConfluenceFetcher {
    username: String,
    api_token: String,
    base_url: String,
    page_id: String,
}

impl ConfluenceFetcher {
    pub fn new(username: String, api_token: String, base_url: String, page_id: String) -> Self {
        ConfluenceFetcher {
            username,
            api_token,
            base_url,
            page_id,
        }
    }
}

impl Fetcher for ConfluenceFetcher {
    fn fetch(&self) -> Result<Vec<domain::KnownAcronym>, Box<dyn Error>> {
        return Ok(vec![
            domain::KnownAcronym::new(
                "API".to_string(),
                "Application Programming Interface".to_string(),
            ),
            domain::KnownAcronym::new("CLI".to_string(), "Command Line Interface".to_string()),
            domain::KnownAcronym::new("GUI".to_string(), "Graphical User Interface".to_string()),
        ]);
    }
}
