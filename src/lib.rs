mod domain;

enum OutputFormat {
    CLI,
    TEXT,
    JSON,
}

struct ResultFormat {
    pub numbering: bool,
    pub format: OutputFormat,
}

pub fn run() {
    let acronym =
        domain::KnownAcronym::new("TDD".to_string(), "Test Driven Development".to_string());
    println!("Acronym: {} - {}", acronym.abbreviation, acronym.definition);
}
