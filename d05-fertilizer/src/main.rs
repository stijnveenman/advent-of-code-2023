#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
struct Mapper {
    source: Range<u64>,
    destination: Range<u64>,
}

#[derive(Debug)]
struct GameData {
    seeds: Vec<u64>,
    mappers: Vec<Vec<Mapper>>,
}

fn parse_input(s: &str) -> GameData {
    let mut iter = s.split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mappers = iter
        .map(|b| {
            b.lines()
                .skip(1)
                .map(|l| {
                    let n = l.split(' ').collect::<Vec<_>>();
                    let des_start = n.first().unwrap().parse::<u64>().unwrap();
                    let source_start = n.get(1).unwrap().parse::<u64>().unwrap();
                    let len = n.get(2).unwrap().parse::<u64>().unwrap();
                    Mapper {
                        source: (source_start..source_start + len),
                        destination: (des_start..des_start + len),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?} {:?}", seeds, mappers);

    GameData { seeds, mappers }
}

impl Mapper {
    fn map(&self, i: u64) -> u64 {
        let diff = dbg!(i) - dbg!(self.source.start);

        self.destination.start + diff
    }
}

fn process(s: &str) -> u64 {
    let input = parse_input(s);

    input
        .seeds
        .into_iter()
        .map(|s| {
            input.mappers.iter().fold(s, |seed, mappers| {
                let mapper = mappers.iter().find(|m| m.source.contains(&seed));
                let Some(mapper) = mapper else { return seed };

                mapper.map(seed)
            })
        })
        .min()
        .unwrap()
}

fn process2(s: &str) -> u64 {
    todo!()
}

static TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 35)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
