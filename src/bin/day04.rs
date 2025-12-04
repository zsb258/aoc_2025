fn main() {
    const INPUT: &str = include_str!("../inputs/day04.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn adj(i: &usize, j: &usize, n: &usize) -> Vec<(usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(|(di, dj)| {
        let ni = *i as isize + di;
        let nj = *j as isize + dj;
        if (ni >= 0 && ni < *n as isize) && (nj >= 0 && nj < *n as isize) {
            Some((ni as usize, nj as usize))
        } else {
            None
        }
    })
    .collect()
}

fn remove_once(grid: &mut Vec<Vec<char>>) -> u64 {
    // assume square grid
    let n = grid.len();
    let mut removed: Vec<(usize, usize)> = vec![];

    let num_removed = (0..n)
        .map(|i| {
            (0..n)
                .filter(|&j| {
                    let to_remove = grid[i][j] == '@'
                        && adj(&i, &j, &n)
                            .iter()
                            .filter(|(ni, nj)| grid[*ni][*nj] == '@')
                            .count()
                            < 4;
                    if to_remove {
                        removed.push((i, j));
                    }
                    to_remove
                })
                .count() as u64
        })
        .sum();

    removed.iter().for_each(|(i, j)| {
        grid[*i][*j] = '.';
    });

    num_removed
}

fn part1(input: &str) -> u64 {
    remove_once(&mut parse(input))
}

fn part2(input: &str) -> u64 {
    (0..)
        .scan(&mut parse(input), |grid, _| {
            let n_removed = remove_once(grid);
            if n_removed > 0 { Some(n_removed) } else { None }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(part1(example), 13);
        assert_eq!(part2(example), 43);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day04.txt");
        assert_eq!(part1(INPUT), 1437);
        assert_eq!(part2(INPUT), 8765);
    }
}
