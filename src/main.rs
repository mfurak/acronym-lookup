use acronym_lookup::{run, Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    run(&cli);
}
