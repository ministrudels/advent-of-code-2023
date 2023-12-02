use std::env;
use std::io::{Error, ErrorKind};

mod day_1;
mod day_2;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "No day specified "));
    }
    let day = args[1].as_str();

    match day {
        "day_1" => day_1::main(),
        "day_2" => day_2::main(),
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Day {} not implemented", day),
            ))
        }
    }
}
