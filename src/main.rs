use std::fs::File;
use std::io::{BufRead, Error, ErrorKind};
use std::{env, io};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_x;

// utility function to read input files
fn read_input_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect::<Result<_, _>>()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "No day specified "));
    }
    let day = args[1].as_str();

    match day {
        "day_x" => day_x::main(read_input_file("src/day_x/input.txt").unwrap()),
        "day_1" => day_1::main(),
        "day_2" => day_2::main(),
        "day_3" => day_3::main(),
        "day_4" => day_4::main(),
        "day_5" => day_5::main(read_input_file("src/day_5/input.txt").unwrap()),
        "day_6" => day_6::main(read_input_file("src/day_6/input.txt").unwrap()),
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Day {} not implemented", day),
            ))
        }
    }
}
