use clap::{Arg, App};

fn main() {
    let matches = App::new("测试程序")
        .version("0.1.0")
        .author("x24 x2x4com@gmail.com")
        .about("argument parsing")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less than your favorite number"))
        .get_matches();

    let my_file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", my_file);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
