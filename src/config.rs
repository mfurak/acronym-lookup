use crate::output::OutputStyle;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct CliParameters {
    pub acronym: String,
    #[arg(short, long, value_enum)]
    pub format: Option<OutputStyle>,
}

pub struct ConfluenceEnvParameters {
    pub user_name: String,
    pub api_token: String,
    pub base_url: String,
    pub page_id: String,
}

pub struct EnvParameters {
    pub confluence: ConfluenceEnvParameters,
    pub file_paths: Option<Vec<String>>,
}

impl EnvParameters {
    #[must_use]
    pub fn load() -> Self {
        Self {
            confluence: ConfluenceEnvParameters {
                user_name: std::env::var("AL_CONFLUENCE_USER_NAME")
                    .expect("AL_CONFLUENCE_USER_NAME is not set"),
                api_token: std::env::var("AL_CONFLUENCE_API_TOKEN")
                    .expect("AL_CONFLUENCE_API_TOKEN is not set"),
                base_url: std::env::var("AL_CONFLUENCE_BASE_URL")
                    .expect("AL_CONFLUENCE_BASE_URL is not set"),
                page_id: std::env::var("AL_CONFLUENCE_PAGE_ID")
                    .expect("AL_CONFLUENCE_PAGE_ID is not set"),
            },
            file_paths: match std::env::var("AL_FILE_PATHS") {
                Ok(file_paths) => {
                    if !file_paths.trim().is_empty() {
                        Some(
                            file_paths
                                .split(',')
                                .filter_map(|s| {
                                    if !s.trim().is_empty() {
                                        Some(s.to_owned())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<String>>(),
                        )
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
        }
    }
}
