pub fn day5() {
    let input = include_str!("day5.txt");

    let present_seat_ids: Vec<usize> = input
        .split_terminator("\n")
        .map(seat)
        .map(seat_id)
        .collect();

    let part_one_result = present_seat_ids.iter().max().expect("no input");
    println!("Part one: {}", part_one_result);

    let min_seat_id = present_seat_ids.iter().min().expect("no input");
    let sorted_seat_ids = sorted(&present_seat_ids);
    for (i, seat_id) in sorted_seat_ids.iter().enumerate() {
        let expected = i + min_seat_id;
        if seat_id != &expected {
            println!("Part two: {}", seat_id - 1);
            break;
        }
    }
}

fn seat_id(pair: (usize, usize)) -> usize {
    let (row, column) = pair;
    row * 8 + column
}

fn seat(sequence: &str) -> (usize, usize) {
    let row = &sequence[0..7];
    let column = &sequence[7..10];
    (decode(row), decode(column))
}

fn decode(sequence: &str) -> usize {
    sequence.chars().rev().enumerate().map(convert).sum()
}

fn convert(pair: (usize, char)) -> usize {
    let (pos, letter) = pair;
    match letter {
        'F' | 'L' => 0,
        'B' | 'R' => 1 << pos,
        _ => panic!("Unexpected letter"),
    }
}

fn sorted<T>(vec: &Vec<T>) -> Vec<T>
where
    T: Clone + Ord,
{
    let mut new_vec = vec.clone();
    new_vec.sort_unstable();
    new_vec
}
