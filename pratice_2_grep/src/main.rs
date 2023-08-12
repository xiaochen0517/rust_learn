use pratice_2_grep::Config;

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(err) = pratice_2_grep::run(config) {
        eprintln!("run grep error: {}", err);
        std::process::exit(1);
    }
}
