use itertools::Itertools;
use std::collections::HashSet;
use std::vec::IntoIter;

fn main() {
    const INPUT: &str = include_str!("../inputs/day08.txt");
    println!("Part1: {}", part1(INPUT, 1000));
    println!("Part2: {}", part2(INPUT));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl Coord {
    fn from_tuple(t: (u64, u64, u64)) -> Self {
        Coord {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }

    fn dist_sq(&self, other: &Coord) -> u64 {
        ((self.x as i64 - other.x as i64).pow(2)
            + (self.y as i64 - other.y as i64).pow(2)
            + (self.z as i64 - other.z as i64).pow(2)) as u64
    }
}

fn parse(input: &str) -> IntoIter<Vec<Coord>> {
    input
        .lines()
        .map(|line| {
            Coord::from_tuple(
                line.splitn(3, ',')
                    .map(|num| num.parse::<u64>().unwrap())
                    .take(3)
                    .collect_tuple::<(u64, u64, u64)>()
                    .unwrap(),
            )
        })
        .combinations(2)
        .sorted_by(|a, b| a[0].dist_sq(&a[1]).cmp(&b[0].dist_sq(&b[1])))
}

fn update_sets(sets: &mut Vec<HashSet<Coord>>, pair: &Vec<Coord>) -> () {
    let a = &pair[0];
    let b = &pair[1];
    let set_a_idx = sets.iter().position(|s| s.contains(&a));
    let set_b_idx = sets.iter().position(|s| s.contains(&b));

    match (set_a_idx, set_b_idx) {
        (Some(ai), Some(bi)) if ai != bi => {
            let setb = sets.remove(bi);
            let ai_new = if bi < ai { ai - 1 } else { ai };
            sets[ai_new].extend(setb);
        }
        (Some(ai), None) => {
            sets[ai].insert(b.clone());
        }
        (None, Some(bi)) => {
            sets[bi].insert(a.clone());
        }
        (None, None) => {
            let mut s = HashSet::new();
            s.insert(a.clone());
            s.insert(b.clone());
            sets.push(s);
        }
        (Some(_), Some(_)) => {}
    }
}

fn part1(input: &str, n_conn: usize) -> u64 {
    let mut sets: Vec<HashSet<Coord>> = Vec::new();

    parse(input).take(n_conn).for_each(|pair| {
        update_sets(&mut sets, &pair);
    });

    sets.iter()
        .sorted_by_key(|s| s.len())
        .rev()
        .take(3)
        .map(|s| s.len() as u64)
        .product()
}

fn part2(input: &str) -> u64 {
    let n = input.lines().count();
    let mut sets: Vec<HashSet<Coord>> = Vec::new();

    parse(input)
        .find_map(|pair| {
            update_sets(&mut sets, &pair);

            if sets.iter().map(|s| s.len()).max().unwrap_or(0) == n {
                Some(&pair[0].x * &pair[1].x)
            } else {
                None
            }
        })
        .expect("Assume will produce valid single set")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part1(example, 10), 40);
        assert_eq!(part2(example), 25272);
    }

    /// verify answers during refactoring
    #[test]
    fn answer() {
        const INPUT: &str = include_str!("../inputs/day08.txt");
        assert_eq!(part1(INPUT, 1000), 97384);
        assert_eq!(part2(INPUT), 9003685096);
    }
}
