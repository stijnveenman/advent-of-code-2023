use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<HashMap<String, u32>>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rounds) = s.split_once(':').ok_or(anyhow!("split"))?;
        let (_, game) = game.split_once(' ').ok_or(anyhow!("game split"))?;

        let rounds = rounds
            .split(';')
            .map(|round| {
                let items = round.split(',');

                let mut h = HashMap::new();

                items.for_each(|item| {
                    let (count, item) = item.trim().split_once(' ').unwrap();
                    h.insert(item.to_string(), count.parse().unwrap());
                });

                h
            })
            .collect::<Vec<_>>();

        Ok(Game {
            id: game.parse()?,
            rounds,
        })
    }
}

fn valid_game(game: &Game) -> bool {
    let mut h = HashMap::new();

    game.rounds.iter().for_each(|round| {
        round.iter().for_each(|(key, value)| {
            let cur = h.get(key).unwrap_or(&0u32);

            if value > cur {
                h.insert(key, *value);
            }
        })
    });

    if let Some(v) = h.get(&"red".to_string()) {
        if *v > 12 {
            return false;
        }
    }
    if let Some(v) = h.get(&"green".to_string()) {
        if *v > 13 {
            return false;
        }
    }
    if let Some(v) = h.get(&"blue".to_string()) {
        if *v > 14 {
            return false;
        }
    }

    true
}

fn process(s: &str) -> u32 {
    let games = s
        .lines()
        .map(|s| s.parse::<Game>().unwrap())
        .filter(valid_game)
        .collect::<Vec<_>>();

    println!("{:?}", games);

    games.into_iter().map(|g| g.id).sum()
}

#[test]
fn test_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    let result = process(input);

    assert_eq!(result, 8);
}
