use acronym_lookup::{config::CliParameters, run};
use clap::Parser;

fn main() {
    let cli = CliParameters::parse();
    run(&cli);
}
