const WIDTH: usize = 31;
const HEIGHT: usize = 323;
type Map = [[bool; WIDTH]; HEIGHT];

pub fn day3() {
    let input = include_str!("day3.txt");
    let mut map: Map = [[false; WIDTH]; HEIGHT];
    for (i, line) in input.lines().enumerate() {
        for (j, letter) in line.chars().enumerate() {
            if letter == '#' {
                map[i][j] = true;
            }
        }
    }

    let part_one_result = num_trees_encountered(map, (3, 1));
    println!("Part one: {}", part_one_result);

    let traversals = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part_two_result: usize = traversals
        .iter()
        .map(|&traversal| num_trees_encountered(map, traversal))
        .product();
    println!("Part two: {}", part_two_result);
}

fn num_trees_encountered(map: Map, traversal: (usize, usize)) -> usize {
    let num_locations = div_up(HEIGHT, traversal.1);
    let mut locations: [(usize, usize); HEIGHT] = [(0, 0); HEIGHT];
    let mut x = 0;
    let mut y = 0;
    for i in 0..num_locations {
        locations[i] = (x, y);
        x = (x + traversal.0) % WIDTH;
        y += traversal.1;
    }

    let num_trees = locations
        .iter()
        .map(|(x, y)| map[*y][*x])
        .filter(|&b| b)
        .count();

    return num_trees;
}

fn div_up(a: usize, b: usize) -> usize {
    (0..a).step_by(b).size_hint().0
}
