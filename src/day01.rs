use std::num::ParseIntError;

use fnv::FnvHashSet;

#[aoc_generator(day01)]
fn parse(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input
        .lines()
        .flat_map(|line| {
            line.split(',').map(|num| {
                let num = num.trim().trim_start_matches('+');
                num.parse::<i64>()
            })
        })
        .collect()
}

#[aoc(day01, part1)]
fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day01, part2)]
fn part2(input: &[i64]) -> i64 {
    let mut sum = 0;
    let mut encountered_freqs = FnvHashSet::from_iter(std::iter::once(sum));

    loop {
        for delta in input {
            sum += delta;
            if !encountered_freqs.insert(sum) {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse("+1, +1, +1").unwrap()), 3);
        assert_eq!(part1(&parse("+1, +1, -2").unwrap()), 0);
        assert_eq!(part1(&parse("-1, -2, -3").unwrap()), -6);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse("+1, -1").unwrap()), 0);
        assert_eq!(part2(&parse("+3, +3, +4, -2, -4").unwrap()), 10);
        assert_eq!(part2(&parse("-6, +3, +8, +5, -6").unwrap()), 5);
        assert_eq!(part2(&parse("+7, +7, -2, -7, -4").unwrap()), 14);
    }
}
