use acronym_lookup::{run, CliParameters};
use clap::Parser;

fn main() {
    let cli = CliParameters::parse();
    run(&cli);
}
