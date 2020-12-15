use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    match arg {
        Some(x) => match x.as_str() {
            "1" => day1::day1(),
            "2" => day2::day2(),
            _ => println!("Unsupported day"),
        },
        None => println!("You must specify a day number."),
    }
}
