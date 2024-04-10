use crate::domain::{AcronymResult, TargetAcronym};

const BOLD_START: &str = "\x1B[1m";
const ITALIC_START: &str = "\x1B[3m";
const TEXT_END: &str = "\x1B[0m";

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum OutputStyle {
    CLI,
    TEXT,
    JSON,
}

pub struct OutputFormat {
    pub numbering: bool,
    pub format: OutputStyle,
}

impl OutputFormat {
    pub fn print_output(&self, results: &[AcronymResult], target_acronym: &TargetAcronym) {
        match self.format {
            OutputStyle::CLI => self.print_cli(results, target_acronym),
            OutputStyle::TEXT => self.print_text(results),
            OutputStyle::JSON => self.print_json(results),
        }
    }

    fn print_cli(&self, results: &[AcronymResult], target_acronym: &TargetAcronym) {
        for result in results {
            let formatted_acronym = format!("{BOLD_START}{}{TEXT_END}", target_acronym.value);
            let formatted_acronym = result
                .acronym
                .abbreviation
                .replace(&target_acronym.value, &formatted_acronym);
            let formatted_definition =
                format!("{ITALIC_START}{}{TEXT_END}", result.acronym.definition);
            println!(
                "{} - {ITALIC_START}{}{TEXT_END}",
                formatted_acronym, formatted_definition
            )
        }
    }

    fn print_text(&self, results: &[AcronymResult]) {
        for result in results {
            println!(
                "{} - {}",
                result.acronym.abbreviation, result.acronym.definition
            )
        }
    }

    fn print_json(&self, results: &[AcronymResult]) {
        let output = serde_json::to_string(results).unwrap_or("Something went wrong".to_string());
        println!("{output}");
    }
}
