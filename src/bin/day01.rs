const INPUT: &str = include_str!("../../inputs/day01.txt");

fn main() {
    println!("Part1: {}", part1(""));
}

fn part1(_input: &str) -> i32 {
    1
}


#[test]
fn example() {
    let example: &str = "";
    assert_eq!(part1(example), 1);
}
