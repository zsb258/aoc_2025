fn main() {
    const INPUT: &str = include_str!("../inputs/day06.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

#[derive(Debug)]
enum Op {
    Plus,
    Mult,
}

#[derive(Debug)]
struct Problem {
    nums: Vec<u64>,
    op: Op,
}

impl Problem {
    fn calculate(&self) -> u64 {
        match self.op {
            Op::Plus => self.nums.iter().sum(),
            Op::Mult => self.nums.iter().product(),
        }
    }
}

fn parse_part1(input: &str) -> Vec<Problem> {
    let tmp: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    (0..tmp[0].len())
        .map(|i| {
            let nums = tmp
                .iter()
                .take(tmp.len() - 1)
                .map(|line| line[i].parse::<u64>().unwrap())
                .collect();
            let op = match tmp.last().unwrap()[i].chars().next().unwrap() {
                '+' => Op::Plus,
                '*' => Op::Mult,
                _ => unreachable!(),
            };

            Problem { nums, op }
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    parse_part1(input).iter().map(Problem::calculate).sum()
}

fn parse_part2(input: &str) -> Vec<Problem> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_nums = grid.len() - 1;

    let mut idx_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    // assume first char is always op
    for (i, c) in grid[n_nums].iter().enumerate().skip(1) {
        if !c.is_whitespace() {
            idx_ranges.push((start, i - 1 - 1)); // move left 1 more for the column separator space
            start = i;
        }
    }
    idx_ranges.push((start, grid[n_nums].len() - 1));

    idx_ranges
        .into_iter()
        .map(|(l, r)| {
            let nums: Vec<u64> = (l..=r)
                .map(|j| {
                    (0..n_nums)
                        .map(|i| grid[i][j])
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect();

            let op = match grid[n_nums][l] {
                '+' => Op::Plus,
                '*' => Op::Mult,
                _ => unreachable!(),
            };

            Problem { nums, op }
        })
        .collect()
}

fn part2(input: &str) -> u64 {
    parse_part2(input).iter().map(Problem::calculate).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(part1(example), 4277556);
        assert_eq!(part2(example), 3263827);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day06.txt");
        assert_eq!(part1(INPUT), 4693159084994);
        assert_eq!(part2(INPUT), 11643736116335);
    }
}
