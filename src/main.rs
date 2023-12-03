mod day1;
mod day2;
mod day3;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    day: String,
    input: Option<String>
}

fn run_day(day_module: &dyn Fn(&str) -> (i64, i64), input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("correct input");
    let (part1_answer, part2_answer) = day_module(&input);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}

fn main() {
    let args = Args::parse();

    let default_input_path = format!("./input/{}.txt", args.day);
    let input_path = args.input.unwrap_or_else(|| default_input_path);

    match args.day.as_str() {
        "day1" => run_day(&day1::solve, &input_path),
        "day2" => run_day(&day2::solve, &input_path),
        "day3" => run_day(&day3::solve, &input_path),
        _ => eprintln!("Invalid day specified."),
    }
}

