use regex::Regex;

pub fn day2() {
    let input = include_str!("day2.txt");
    let re =
        Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<letter>[a-z]{1}): (?P<password>[a-z]+)\n")
            .unwrap();

    let mut passwords: Vec<Password> = Vec::with_capacity(1000);
    passwords.extend(re.captures_iter(input).map(|cap| Password {
        first: cap.name("first").unwrap().as_str().parse().unwrap(),
        second: cap.name("second").unwrap().as_str().parse().unwrap(),
        letter: cap.name("letter").unwrap().as_str().chars().next().unwrap(),
        password: cap.name("password").unwrap().as_str().to_string(),
    }));

    let num_valid_part_one = passwords
        .iter()
        .filter(|password| password.is_valid_part_one())
        .count();

    println!("Part One: Number valid = {}", num_valid_part_one);

    let num_valid_part_two = passwords
        .iter()
        .filter(|password| password.is_valid_part_two())
        .count();

    println!("Part Two: Number valid = {}", num_valid_part_two);
}

#[derive(Debug)]
struct Password {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid_part_one(&self) -> bool {
        let num_letters = Password::num_letters(self);
        self.first <= num_letters && num_letters <= self.second
    }

    fn is_valid_part_two(&self) -> bool {
        let first_character = match self.password.chars().nth(self.first - 1) {
            Some(x) => x,
            None => return false,
        };
        let second_character = match self.password.chars().nth(self.second - 1) {
            Some(x) => x,
            None => return false,
        };
        let is_first_valid = first_character == self.letter;
        let is_second_valid = second_character == self.letter;
        is_first_valid ^ is_second_valid
    }

    fn num_letters(&self) -> usize {
        self.password.chars().filter(|&c| c == self.letter).count()
    }
}
