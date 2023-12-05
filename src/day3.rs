use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Position = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    value: u32,
    positions: Vec<Position>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn is_symbol(c: char) -> bool {
    (!c.is_ascii_digit()) && c != '.'
}

fn get_from_pos(p: Position, map: &[Vec<char>]) -> char {
    map[p.0][p.1]
}

fn get_neighbours(p: Position, map: &[Vec<char>]) -> Vec<Position> {
    let mut neighbours = Vec::new();
    let max_y = map.len();
    let max_x = map[0].len();

    if let Some(yprev) = p.0.checked_sub(1) {
        if let Some(xprev) = p.1.checked_sub(1) {
            neighbours.push((yprev, xprev));
        }
        neighbours.push((yprev, p.1));
        if (p.1 + 1) < max_x {
            neighbours.push((yprev, p.1 + 1));
        }
    }
    if let Some(xprev) = p.1.checked_sub(1) {
        neighbours.push((p.0, xprev));
    }

    if (p.1 + 1) < max_x {
        neighbours.push((p.0, p.1 + 1));
    }

    if (p.0 + 1) < max_y {
        if let Some(xprev) = p.1.checked_sub(1) {
            neighbours.push((p.0 + 1, xprev));
        }
        neighbours.push((p.0 + 1, p.1));
        if (p.1 + 1) < max_x {
            neighbours.push((p.0 + 1, p.1 + 1));
        }
    }

    neighbours
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u32 {
    let mut numbers = Vec::new();
    for (y, l) in input.iter().enumerate() {
        let mut in_number = false;
        let mut curval = 0;
        let mut positions = Vec::new();
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                curval = (curval * 10) + c.to_digit(10).unwrap();
                positions.push((y, x));
                in_number = true;
            } else {
                if in_number {
                    numbers.push(Number {
                        value: curval,
                        positions,
                    })
                }
                curval = 0;
                positions = Vec::new();
                in_number = false;
            }
        }
        if in_number {
            numbers.push(Number {
                value: curval,
                positions,
            })
        }
    }

    // dbg!(&numbers);
    numbers
        .iter()
        .filter_map(|n| {
            if n.positions.iter().any(|p| {
                get_neighbours(*p, input)
                    .iter()
                    .any(|np| is_symbol(get_from_pos(*np, input)))
            }) {
                Some(n.value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    let mut numbers = Vec::new();
    let mut numbers_table: HashMap<Position, Number> = HashMap::new();

    for (y, l) in input.iter().enumerate() {
        let mut in_number = false;
        let mut curval = 0;
        let mut positions = Vec::new();
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                curval = (curval * 10) + c.to_digit(10).unwrap();
                positions.push((y, x));
                in_number = true;
            } else {
                if in_number {
                    let n = Number {
                        value: curval,
                        positions: positions.clone(),
                    };
                    numbers.push(n.clone());
                    for p in positions {
                        numbers_table.insert(p, n.clone());
                    }
                }
                curval = 0;
                positions = Vec::new();
                in_number = false;
            }
        }
        if in_number {
            let n = Number {
                value: curval,
                positions: positions.clone(),
            };
            numbers.push(n.clone());
            for p in positions {
                numbers_table.insert(p, n.clone());
            }
        }
    }

    let mut total = 0;
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '*' {
                let mut nums = get_neighbours((y, x), input)
                    .iter()
                    .filter_map(|p| numbers_table.get(p).map(|x| x.clone()))
                    .collect::<Vec<Number>>();

                nums.dedup();
                if nums.len() == 2 {
                    total += nums[0].value * nums[1].value;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part1(&input_generator(inp)), 4361);
    }

    #[test]
    fn test_3() {
        let inp = "......=.........................=........733..433...128..............*.......889...........................661.........-358...$....281......";

        assert_eq!(solve_part1(&input_generator(inp)), 358);
    }
    #[test]
    fn test_4() {
        let inp = "...=5";

        assert_eq!(solve_part1(&input_generator(inp)), 5);
    }

    #[test]
    fn test_5() {
        let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part2(&input_generator(inp)), 467835);
    }
}
