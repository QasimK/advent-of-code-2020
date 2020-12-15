pub fn day1() {
    let input = include_str!("day1.txt");
    let mut numbers: Vec<i32> = Vec::with_capacity(200);
    numbers.extend(
        input
            .split_whitespace()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .filter(|&x| 0 <= x && x < 2020),
    );
    numbers.sort_unstable();
    numbers.dedup();

    // Part 1
    match get_pair_sum(&numbers, 2020) {
        Some((x, y)) => {
            let answer = x * y;
            println!(
                "Part One: {x} + {y} = 2020; {x} x {y} = {answer}",
                x = x,
                y = y,
                answer = answer
            );
        }
        None => std::process::exit(1),
    }

    // Part 2
    for (i, x) in numbers.iter().enumerate() {
        let slice = &numbers[i + 1..];
        match get_pair_sum(slice, 2020 - x) {
            Some((y, z)) => {
                let answer = x * y * z;
                println!(
                    "Part Two: {x} + {y} + {z} = 2020; {x} x {y} x {z} = {answer}",
                    x = x,
                    y = y,
                    z = z,
                    answer = answer
                );
                std::process::exit(0);
            }
            None => {}
        }
    }

    std::process::exit(1);
}

fn get_pair_sum(numbers: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut max_n = numbers.len();
    for (i, x) in numbers.iter().enumerate() {
        if i >= max_n {
            break;
        }
        let slice = &numbers[i + 1..max_n];
        for y in slice.iter().rev() {
            if x + y > target {
                // y is too big, no need to ever go to this big y again.
                max_n -= 1;
            } else if x + y == target {
                return Some((*x, *y));
            } else if x + y < target {
                // x & y are both too small now
                break;
            }
        }
    }

    None
}
