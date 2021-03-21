use std::env;
use std::process;
use minigrep::{Config, test1};

fn main() {
    let v = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
    let a = test1(&v);
    println!("dd: {}", a);

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let (query, filename) = parse_config(&args);
    //let config = Config::new(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprint!("Problem parsing args: {}\n", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}



