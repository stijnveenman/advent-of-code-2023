#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use aoc_toolbox::{char_grid::CharGrid, point::Point};
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> Vec<LineItem> {
    s.lines()
        .map(|l| {
            let mut i = l.split(' ');
            LineItem {
                dir: i.next().unwrap().chars().next().unwrap(),
                count: i.next().unwrap().parse().unwrap(),
                hex: i.next().unwrap().replace(['(', ')'], ""),
            }
        })
        .collect()
}

#[derive(Debug)]
struct LineItem {
    dir: char,
    count: isize,
    hex: String,
}

impl LineItem {
    fn steps(&self, p: &Point) -> (Vec<Point>, Point) {
        match self.dir {
            'R' => {
                let v = (p.x..=p.x + self.count)
                    .map(|x| Point::new(x, p.y))
                    .collect();
                let n = Point::new(p.x + self.count, p.y);
                (v, n)
            }
            'L' => {
                let v = (p.x - self.count..=p.x)
                    .map(|x| Point::new(x, p.y))
                    .collect();
                let n = Point::new(p.x - self.count, p.y);
                (v, n)
            }
            'D' => {
                let v = (p.y..=p.y + self.count)
                    .map(|y| Point::new(p.x, y))
                    .collect();
                let n = Point::new(p.x, p.y + self.count);
                (v, n)
            }
            'U' => {
                let v = (p.y - self.count..=p.y)
                    .map(|y| Point::new(p.x, y))
                    .collect();
                let n = Point::new(p.x, p.y - self.count);
                (v, n)
            }

            _ => panic!("shouldnt reach"),
        }
    }
}

fn find_inside<T>(grid: &CharGrid<T>) -> Option<Point> {
    let upper = grid.upper();

    for y in 0..=upper.y {
        let mut crossings = 0;
        for x in 0..upper.x {
            match grid.get_xy(x, y) {
                Some(_) => {
                    crossings += 1;
                    if crossings > 1 {
                        break;
                    }
                }
                None => {
                    if crossings == 1 {
                        return Some(Point::new(x, y));
                    }
                }
            }
        }
    }

    None
}

fn process(s: &str) -> usize {
    let input = parse(s);
    let mut grid = CharGrid::new("", |c| Some(""));

    let mut current = Point::new(0, 0);
    for item in input.iter() {
        let (next_points, next) = item.steps(&current);
        current = next;

        next_points.into_iter().for_each(|p| {
            grid.set(p, item.hex.as_ref());
        })
    }

    grid.recalculate_bounds();
    grid.draw_existing();

    let inside = find_inside(&grid).unwrap();
    println!("{:?}", inside);

    grid.floodfill(&inside, "");
    grid.draw_existing();

    grid.points().count()
}

fn process2(s: &str) -> usize {
    todo!()
}

static TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 62)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
