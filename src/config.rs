use crate::output::OutputStyle;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct CliParameters {
    pub acronym: String,
    #[arg(short, long, value_enum)]
    pub format: Option<OutputStyle>,
}
