pub fn day4() {
    let input = include_str!("day4.txt");

    let part_one_result = input
        .split("\n\n")
        .filter(|passport| are_required_fields_present(passport))
        .count();
    println!("Part one: {}", part_one_result);

    let part_two_result = input
        .split("\n\n")
        .filter(|passport| are_required_fields_present(passport))
        .filter(|passport| are_required_fields_valid(passport))
        .count();
    println!("Part two: {}", part_two_result);
}

fn are_required_fields_present(passport: &str) -> bool {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut is_valid = true;
    for field in REQUIRED_FIELDS.iter() {
        if !passport.contains(field) {
            is_valid = false;
            break;
        }
    }
    is_valid
}

fn are_required_fields_valid(passport: &str) -> bool {
    passport
        .split_whitespace()
        .all(|field| is_field_valid(field))
}

fn is_field_valid(field: &str) -> bool {
    let (key, value_colon) = field.split_at(3);
    let value = value_colon.strip_prefix(":").expect("no colon");

    match key {
        "byr" => is_byr_valid(value),
        "iyr" => is_iyr_valid(value),
        "eyr" => is_eyr_valid(value),
        "hgt" => is_hgt_valid(value),
        "hcl" => is_hcl_valid(value),
        "ecl" => is_ecl_valid(value),
        "pid" => is_pid_valid(value),
        _ => true,
    }
}

fn is_byr_valid(raw: &str) -> bool {
    let value: i32 = raw.parse().expect("invalid int");
    1920 <= value && value <= 2002
}

fn is_iyr_valid(raw: &str) -> bool {
    let value: i32 = raw.parse().expect("invalid int");
    2010 <= value && value <= 2020
}

fn is_eyr_valid(raw: &str) -> bool {
    let value: i32 = raw.parse().expect("invalid int");
    2020 <= value && value <= 2030
}

fn is_hgt_valid(raw: &str) -> bool {
    let iend = raw.len();
    if raw.ends_with("cm") {
        let value = raw[..iend - 2].parse().expect("invalid int");
        150 <= value && value <= 193
    } else if raw.ends_with("in") {
        let value = raw[..iend - 2].parse().expect("invalid int");
        59 <= value && value <= 76
    } else {
        false
    }
}

fn is_hcl_valid(raw: &str) -> bool {
    raw.starts_with('#') && raw[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_ecl_valid(raw: &str) -> bool {
    const VALID_VALUES: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    VALID_VALUES.iter().any(|&value| value == raw)
}

fn is_pid_valid(raw: &str) -> bool {
    raw.len() == 9 && raw.chars().all(|c| c.is_ascii_digit())
}
