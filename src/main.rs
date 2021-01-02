use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    match arg {
        Some(x) => match x.as_str() {
            "1" => day1::day1(),
            "2" => day2::day2(),
            "3" => day3::day3(),
            "4" => day4::day4(),
            "5" => day5::day5(),
            _ => println!("Unsupported day"),
        },
        None => println!("You must specify a day number."),
    }
}
