use minigrep_sopesto::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema pasando argumentos: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_sopesto::run(config) {
        eprintln!("Error: {e}");
        process::exit(1)
    }
}
