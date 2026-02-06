use clap::Parser;
use ringo_srs::cli::Cli;

fn main() {
    let cli = Cli::parse();
    match ringo_srs::run(&cli) {
        Ok(json) => {
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        Err(e) => {
            eprintln!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
            std::process::exit(1);
        }
    }
}
