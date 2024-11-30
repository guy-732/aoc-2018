use std::num::ParseIntError;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("+1, +1, +1").unwrap()), 3);
        assert_eq!(part1(&parse("+1, +1, -2").unwrap()), 0);
        assert_eq!(part1(&parse("-1, -2, -3").unwrap()), -6);
    }
}
