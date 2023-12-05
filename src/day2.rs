use aoc_runner_derive::{aoc, aoc_generator};

pub struct Game {
    pub id: u32,
    pub game_sets: Vec<GameSet>,
}

#[derive(Default)]
pub struct GameSet {
    pub green: u32,
    pub blue: u32,
    pub red: u32,
}

fn line_to_game(line: &str) -> Game {
    let mut parts_it = line.split(": ");
    let mut game_it = parts_it.next().unwrap().split(' ');
    game_it.next();
    let id = game_it.next().unwrap().parse().unwrap();

    let sets_it = parts_it.next().unwrap().split("; ");
    let game_sets = sets_it
        .map(|s| {
            let pieces_it = s.split(", ");
            let mut gameset = GameSet::default();

            for p in pieces_it {
                let mut inner_it = p.split(' ');
                let num = inner_it.next().unwrap().parse().unwrap();
                let colour = inner_it.next().unwrap();
                match colour {
                    "green" => gameset.green = num,
                    "red" => gameset.red = num,
                    "blue" => gameset.blue = num,
                    _ => {
                        log::error!("Bad value")
                    }
                }
            }
            gameset
        })
        .collect::<Vec<GameSet>>();

    Game { id, game_sets }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(line_to_game).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter_map(|g| {
            if g.game_sets
                .iter()
                .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
            {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let mut base = GameSet::default();
            let basep = &mut base;
            g.game_sets.iter().fold(basep, |acc, s| {
                acc.green = acc.green.max(s.green);
                acc.red = acc.red.max(s.red);
                acc.blue = acc.blue.max(s.blue);
                acc
            });
            base.green * base.red * base.blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part1(&input_generator(inp)), 8);
    }
    #[test]
    fn test_2() {
        let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part2(&input_generator(inp)), 2286);
    }
}
