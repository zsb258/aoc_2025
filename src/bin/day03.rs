fn main() {
    const INPUT: &str = include_str!("../inputs/day03.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    solve(input, 2)
}

fn part2(input: &str) -> u64 {
    solve(input, 12)
}
fn parse(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect()
    })
}

fn solve(input: &str, n_digits: usize) -> u64 {
    parse(input)
        .map(|nums: Vec<u64>| {
            let mut left_bound = 0;
            (0..n_digits)
                .map(|i| {
                    let right_bound = nums.len() - (n_digits - i - 1);
                    let slice = &nums[left_bound..right_bound];

                    let d = slice.iter().max().unwrap();
                    let idx = slice.iter().position(|v| v == d).unwrap();
                    left_bound = left_bound + idx + 1;

                    d.to_string()
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(part1(example), 357);
        assert_eq!(part2(example), 3121910778619);
    }

    #[test]
    fn custom() {
        assert_eq!(part1("998"), 99);
        assert_eq!(part1("111"), 11);

        assert_eq!(part2("987654321111111"), 987654321111);
        assert_eq!(part2("818181911112111"), 888911112111);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day03.txt");
        assert_eq!(part1(INPUT), 17244);
        assert_eq!(part2(INPUT), 171435596092638);
    }
}
