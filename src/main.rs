mod day02;
mod day01;
mod day03;
mod day04;
mod day05;
mod day06;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let day = 6;
    let solve = match day {
        1 => day01::solve01,
        2 => day02::solve02,
        3 => day03::solve03,
        4 => day04::solve04,
        5 => day05::solve,
        6 => day06::solve,
        _ => {|_| Ok(-1)}
    };
    let filename = format!("assets/input{:02}.txt", day);
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let res = solve(reader.lines())?;
    println!("Day {} solution: {}", day, res);

    // let sol02 = solve02(reader.lines())?;
    Ok(())
}
