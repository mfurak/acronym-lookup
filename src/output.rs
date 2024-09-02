use crate::domain::{AcronymResult, TargetAcronym};

const BOLD_START: &str = "\x1B[1m";
const ITALIC_START: &str = "\x1B[3m";
const TEXT_END: &str = "\x1B[0m";

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Style {
    Cli,
    Text,
    Json,
}

pub struct Format {
    pub format: Style,
}

impl Format {
    pub fn print_output(&self, results: &[AcronymResult], target_acronym: &TargetAcronym) {
        match self.format {
            Style::Cli => Self::print_cli(results, target_acronym),
            Style::Text => Self::print_text(results),
            Style::Json => Self::print_json(results),
        }
    }

    fn print_cli(results: &[AcronymResult], target_acronym: &TargetAcronym) {
        for result in results {
            let formatted_acronym = format!("{BOLD_START}{}{TEXT_END}", target_acronym.value);
            let formatted_acronym = result
                .acronym
                .abbreviation
                .replace(&target_acronym.value, &formatted_acronym);
            let formatted_definition =
                format!("{ITALIC_START}{}{TEXT_END}", result.acronym.definition);
            println!("{formatted_acronym} - {ITALIC_START}{formatted_definition}{TEXT_END}");
        }
    }

    fn print_text(results: &[AcronymResult]) {
        for result in results {
            println!(
                "{} - {}",
                result.acronym.abbreviation, result.acronym.definition
            );
        }
    }

    fn print_json(results: &[AcronymResult]) {
        let output =
            serde_json::to_string(results).unwrap_or_else(|_| "Something went wrong".to_string());
        println!("{output}");
    }
}
