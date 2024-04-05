use acronym_lookup::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let acronym_string = args.get(1).unwrap().to_string();

    run(acronym_string);
}
