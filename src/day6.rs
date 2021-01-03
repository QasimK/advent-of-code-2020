use std::collections::HashSet;

pub fn day6() {
    let input = include_str!("day6.txt");

    let part_one_result: usize = input
        .split("\n\n")
        .map(|section| section.replace("\n", "")) // Vec<String>
        .map(|s| sorted(&s)) // Vec<Vec<char>>
        .map(|v| dedup(&v)) // Vec<Vec<char>>
        .map(|v| v.into_iter().collect::<String>()) // Vec<String>
        .map(|s| s.len()) // Vec<usize>
        .sum();
    println!("Part one: {:?}", part_one_result);

    let _part_two_result: usize = input
        .split("\n\n") // Vec<String>
        .map(num_everyone) // Vec<usize>
        .sum();

    println!("Part two: {:?}", _part_two_result);
}

fn num_everyone(section: &str) -> usize {
    const STRING: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut letters: HashSet<char> = STRING.chars().collect();

    for line in section.lines() {
        let hash: HashSet<char> = line.chars().collect();
        letters.retain(|c| hash.contains(&c));
    }

    letters.len()
}

fn sorted(string: &String) -> Vec<char> {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars
}

fn dedup(v: &Vec<char>) -> Vec<char> {
    let mut new: Vec<char> = v.clone();
    new.dedup();
    new
}
