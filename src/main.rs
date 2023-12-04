use std::env;
use std::io::{Error, ErrorKind};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_x;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "No day specified "));
    }
    let day = args[1].as_str();

    match day {
        "day_x" => day_x::main(),
        "day_1" => day_1::main(),
        "day_2" => day_2::main(),
        "day_3" => day_3::main(),
        "day_4" => day_4::main(),
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Day {} not implemented", day),
            ))
        }
    }
}
