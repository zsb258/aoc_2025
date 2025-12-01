fn main() {
    const INPUT: &str = include_str!("../inputs/day01.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dir = match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => 0,
            };
            let step: i32 = line[1..].parse().unwrap();
            dir * step
        })
        .scan(50, |state, new| {
            *state = (*state + new) % 100;
            Some(*state)
        })
        .filter(|n| *n == 0)
        .count()
        .try_into()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let dir = match line.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => 0,
            };
            let step: i32 = line[1..].parse().unwrap();
            dir * step
        })
        .scan(50, |state, new| {
            let prev = *state;
            let curr = *state + new;
            *state = if curr % 100 < 0 {
                curr % 100 + 100
            } else {
                curr % 100
            };

            if new.abs() == 0 {
                Some(0)
            } else if prev == 0 {
                Some(new.abs() / 100)
            } else {
                match curr {
                    0 => Some(1),
                    n if n > 0 => Some(n / 100),
                    n if n < 0 => Some(1 + n.abs() / 100),
                    _ => unreachable!(),
                }
            }
        })
        .sum::<i32>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(example), 3);
        assert_eq!(part2(example), 6);
    }

    #[test]
    fn custom() {
        assert_eq!(part2("R1"), 0);
        assert_eq!(part2("R50"), 1);
        assert_eq!(part2("R200"), 2);
        assert_eq!(part2("R150"), 2);

        assert_eq!(part2("L1"), 0);
        assert_eq!(part2("L50"), 1);
        assert_eq!(part2("L200"), 2);
        assert_eq!(part2("L150"), 2);

        assert_eq!(part2("R50\nR5"), 1);
        assert_eq!(part2("R50\nR100"), 2);
        assert_eq!(part2("R50\nL100"), 2);
        assert_eq!(part2("R50\nR200"), 3);

        assert_eq!(part2("L50\nL5"), 1);
        assert_eq!(part2("L50\nL100"), 2);
        assert_eq!(part2("L50\nR100"), 2);
        assert_eq!(part2("L50\nL200"), 3);

        assert_eq!(part2("L1050"), 11);

        assert_eq!(part2("L50\nL100\nR100"), 3);
        assert_eq!(part2("L50\nL0\nR0"), 1);
    }
}
