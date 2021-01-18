use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//
//     Config { query, filename }
// }

impl Config {
    pub fn new<'a>(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // let err : &'a String = &format!("not enough args, need 3 get {}", args.len());
            // let err_str : &'static str = err.borrow();
            let err_str: &'static str = "not enough args, need 3 get {}";
            // let err: &'static str = err_str.as_str();
            println!("{}", err_str);
            return Err(err_str)
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn test1<'a>(args: &'a [String]) -> &'a str {
    // let msg = format!("len: {}", args.len());
    let msg: &'a String = String::from("tttt");
    let msg_str : &'a str = msg.as_str();
    msg_str
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename).unwrap_or_else(|err| {
    //     print!("Failed to read file: {}\n", err);
    //     process::exit(1);
    // });

    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}