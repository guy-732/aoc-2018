use itertools::{zip_eq, Itertools};

#[aoc(day02, part1)]
fn part1(input: &str) -> u64 {
    let mut occurences = [0_u32; 256];

    let (appears_twice, appears_thrice) = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(str::trim)
        .map(|line: &str| {
            line.as_bytes()
                .iter()
                .for_each(|&c| occurences[c as usize] += 1);
            let twice = occurences.iter().any(|&n| n == 2);
            let thrice = occurences.iter().any(|&n| n == 3);

            occurences.fill(0);
            (twice as u64, thrice as u64)
        })
        .reduce(|(acc1, acc2), (el1, el2)| (acc1 + el1, acc2 + el2))
        .expect("No lines found");

    appears_twice * appears_thrice
}

fn differs_by_one_char(elements: Vec<&str>) -> Option<String> {
    let mut diff_by_one = false;
    let mut result = String::new();
    for (&c1, &c2) in zip_eq(elements[0].as_bytes(), elements[1].as_bytes()) {
        if c1 != c2 {
            if !diff_by_one {
                diff_by_one = true;
            } else {
                return None;
            }
        } else {
            result.push(c1 as char);
        }
    }

    // should not happen
    if !diff_by_one {
        eprintln!("2 strings with same value ({:?})", result);
        None
    } else {
        Some(result)
    }
}

#[aoc(day02, part2)]
fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(str::trim)
        .combinations(2)
        .find_map(differs_by_one_char)
        .expect("Could not find match")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str = "
    abcdef
    bababc
    abbcde
    abcccd
    aabcdd
    abcdee
    ababab";

    #[test]
    fn part1_example() {
        assert_eq!(part1(PART1_EXAMPLE), 12);
    }

    const PART2_EXAMPLE: &str = "
    abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz";

    #[test]
    fn part2_example() {
        assert_eq!(part2(PART2_EXAMPLE), "fgij");
    }
}
