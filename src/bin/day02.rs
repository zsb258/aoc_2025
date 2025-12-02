fn main() {
    const INPUT: &str = include_str!("../inputs/day02.txt");
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn parse(input: &str) -> impl Iterator<Item = u64> {
    input.lines().next().unwrap().split(",").flat_map(|pair| {
        let (start, end) = pair.split_once("-").unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    })
}

fn part1(input: &str) -> u64 {
    parse(input)
        .filter_map(|num| {
            let num_str: String = num.to_string();
            let mid = &num_str.len() / 2;
            if &num_str[..mid] == &num_str[mid..] {
                Some(num_str.parse::<u64>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    parse(input)
        .filter_map(|num| {
            let num_str: String = num.to_string();
            let mid = &num_str.len() / 2;
            let invalid = (1..=mid)
                .map(|i| {
                    let mul = &num_str.len() / i;
                    let repeated: String = num_str[0..i].repeat(mul);
                    &repeated == &num_str
                })
                .any(|b| b);
            if invalid {
                Some(num_str.parse::<u64>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(example), 1227775554);
        assert_eq!(part2(example), 4174379265);
    }

    #[test]
    fn custom() {
        assert_eq!(part1("998-1012"), 1010);
        assert_eq!(part2("998-1012"), 999 + 1010);
    }
}
