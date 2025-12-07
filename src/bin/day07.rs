fn main() {
    const INPUT: &str = include_str!("../inputs/day07.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let s_idx = grid[0].iter().position(|&c| c == 'S').unwrap();
    grid[0][s_idx] = '|';

    grid
}

fn part1(input: &str) -> u64 {
    let mut grid = parse(input);

    let n = grid.len();
    let mut res = 0;

    for i in 1..n {
        for j in 0..grid[i].len() {
            if grid[i - 1][j] == '|' {
                if grid[i][j] == '^' {
                    // assume always have space on left and right - seen in input
                    grid[i][j - 1] = '|';
                    grid[i][j + 1] = '|';
                    res += 1;
                } else {
                    grid[i][j] = '|';
                }
            }
        }
    }

    res
}

fn part2(input: &str) -> u64 {
    let mut grid = parse(input);
    let mut counts: Vec<Vec<u64>> = vec![vec![0; grid[0].len()]; grid.len()];

    let start_j = grid[0].iter().position(|&c| c == '|').unwrap();
    counts[0][start_j] = 1;

    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i - 1][j] == '|' {
                let prev_count = counts[i - 1][j];
                if grid[i][j] == '^' {
                    // assume always have space on left and right - seen in input
                    grid[i][j - 1] = '|';
                    counts[i][j - 1] += prev_count;

                    grid[i][j + 1] = '|';
                    counts[i][j + 1] += prev_count;
                } else {
                    grid[i][j] = '|';
                    counts[i][j] += prev_count;
                }
            }
        }
    }

    counts[grid.len() - 1].iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(example), 21);
        assert_eq!(part2(example), 40);
    }

    #[test]
    fn custom() {
        let example: &str = ".......S.......
...............
.......^.......";
        assert_eq!(part2(example), 2);

        let example: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............";
        assert_eq!(part2(example), 4);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day07.txt");
        assert_eq!(part1(INPUT), 1579);
        assert_eq!(part2(INPUT), 13418215871354);
    }
}
