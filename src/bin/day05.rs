fn main() {
    const INPUT: &str = include_str!("../inputs/day05.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (interval_str, numbers_str) = input.split_once("\n\n").unwrap();
    let intervals: Vec<(u64, u64)> = interval_str
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect();
    let numbers: Vec<u64> = numbers_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (intervals, numbers)
}

fn merge_intervals(intervals: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    intervals.sort_by_key(|&(s, _e)| s);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut curr = intervals[0]; // assume non-empty

    for &next in &intervals[1..] {
        if next.0 <= curr.1 + 1 {
            curr.1 = curr.1.max(next.1);
        } else {
            merged.push(curr);
            curr = next;
        }
    }
    merged.push(curr);

    merged
}

fn part1(input: &str) -> u64 {
    let (intervals, numbers) = parse(input);
    // just brute force
    numbers
        .iter()
        .filter(|&n| intervals.iter().any(|(s, e)| n >= s && n <= e))
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let (mut intervals, _numbers) = parse(input);
    merge_intervals(&mut intervals)
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(part1(example), 3);
        assert_eq!(part2(example), 14);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day05.txt");
        assert_eq!(part1(INPUT), 623);
        assert_eq!(part2(INPUT), 353507173555373);
    }
}
