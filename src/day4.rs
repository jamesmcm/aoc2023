use std::{collections::HashSet, hash::RandomState};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Card {
    pub id: u32,
    pub winning_nums: Vec<u32>,
    pub your_nums: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let mut card_split = l.split(": ");
            let mut start_split = card_split.next().unwrap().split_whitespace();
            start_split.next();
            let id = start_split.next().unwrap().parse().unwrap();

            let mut num_parts = card_split.next().unwrap().split(" | ");
            let win_parts = num_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();
            let your_parts = num_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();

            Card {
                id,
                winning_nums: win_parts,
                your_nums: your_parts,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|c| {
            let winning_nums_set: HashSet<u32, RandomState> =
                HashSet::from_iter(c.winning_nums.iter().cloned());
            let your_nums_set: HashSet<u32> = HashSet::from_iter(c.your_nums.iter().cloned());
            let win_count = HashSet::from_iter(your_nums_set)
                .intersection(&winning_nums_set)
                .count();
            if win_count > 0 {
                2_u32.pow((win_count - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part1(&input_generator(inp)), 13);
    }
}
