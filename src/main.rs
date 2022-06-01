use std::{env, process};

use minigrep::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = Config::new(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: minigrep <query> <file>");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config).await {
        eprintln!("Application error: {:#}", err);
        process::exit(1);
    }
}
