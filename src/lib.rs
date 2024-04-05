mod domain;

enum OutputFormat {
    CLI,
    TEXT,
    JSON,
}

struct ResultFormat {
    numbering: bool,
    format: OutputFormat,
}

pub fn run() {
    let know_acronyms = vec![
        domain::KnownAcronym::new(
            "API".to_string(),
            "Application Programming Interface".to_string(),
        ),
        domain::KnownAcronym::new("CLI".to_string(), "Command Line Interface".to_string()),
        domain::KnownAcronym::new("GUI".to_string(), "Graphical User Interface".to_string()),
    ];

    let target_acronym = domain::TargetAcronym::new("ap".to_string());

    let res = domain::lookup_acronym(&target_acronym, know_acronyms);
    println!("{:?}", res);
}
