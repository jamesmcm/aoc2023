use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| {
            let v = s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();

            v.first().unwrap().to_digit(10).unwrap() * 10 + v.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mod_i = input
        .iter()
        .map(|s| {
            let mut to_insert = (0..9)
                .flat_map(|i| find_prepend_num(s, i, &nums))
                .collect::<Vec<(usize, u32)>>();

            to_insert.sort_by(|x, y| x.0.cmp(&y.0));

            // dbg!(&to_insert);
            let mut new_s = s.clone();
            for (inserted, p) in to_insert.iter().enumerate() {
                new_s = format!(
                    "{}{}{}",
                    &new_s[..(p.0 + inserted)],
                    p.1,
                    &new_s[(p.0 + inserted)..]
                );
            }
            new_s
        })
        .collect::<Vec<String>>();

    // dbg!(&mod_i);
    solve_part1(&mod_i)
}

fn find_prepend_num(s: &str, i: u32, nums: &[&str]) -> Vec<(usize, u32)> {
    let mut res = Vec::new();
    recursive_find(s, i, nums, 0, &mut res);
    res
}

fn recursive_find(s: &str, i: u32, nums: &[&str], prev_ix: usize, res: &mut Vec<(usize, u32)>) {
    if let Some(ix) = s.find(nums[i as usize]) {
        res.push((prev_ix + ix, i + 1));
        recursive_find(&s[ix + 1..], i, nums, prev_ix + ix + 1, res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_part1(&input_generator(inp)), 142);
    }

    #[test]
    fn test_2() {
        let inp = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_part2(&input_generator(inp)), 281);
    }

    #[test]
    fn test_3() {
        let inp = "fourfourlrcn6
dkmmzhbvq3three6threeqgcfgkv";

        assert_eq!(solve_part2(&input_generator(inp)), 46 + 33);
    }
}
