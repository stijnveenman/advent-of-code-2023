use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::point::Point;

pub struct CharGrid<T> {
    map: HashMap<Point, T>,
    upper: Point,
    lower: Point,
}

impl<T> CharGrid<T> {
    pub fn new<F>(s: &str, parse_char: F) -> CharGrid<T>
    where
        F: Fn(char) -> Option<T> + Copy,
    {
        let map = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| Some((Point::new(x as isize, y as isize), parse_char(c)?)))
            })
            .flatten()
            .collect();

        let lower = Point::new(0, 0);
        let upper = Point::new(
            s.lines().next().unwrap_or("").len() as isize - 1,
            s.lines().count() as isize - 1,
        );

        CharGrid { map, lower, upper }
    }

    pub fn is_within(&self, p: &Point) -> bool {
        p.is_within(&self.lower, &self.upper)
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.map.get(p)
    }

    pub fn get_xy(&self, x: isize, y: isize) -> Option<&T> {
        self.map.get(&Point::new(x, y))
    }

    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.map.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point, &T)> {
        self.map.iter()
    }

    pub fn lower(&self) -> Point {
        self.lower
    }

    pub fn upper(&self) -> Point {
        self.upper
    }

    pub fn recalculate_bounds(&mut self) {
        self.upper = Point::new(
            self.map.keys().map(|p| p.x).max().unwrap(),
            self.map.keys().map(|p| p.y).max().unwrap(),
        );
    }

    pub fn set(&mut self, p: Point, item: T) {
        self.map.insert(p, item);
    }

    pub fn floodfill(&mut self, from: &Point, with: T)
    where
        T: Copy,
    {
        let mut togo = vec![*from];

        while let Some(current) = togo.pop() {
            self.set(current, with);

            for n in current.neighbours() {
                if self.get(&n).is_some() {
                    continue;
                }

                togo.push(n);
            }
        }
    }

    pub fn draw<F>(&self, f: F)
    where
        F: Fn(&Point, Option<&T>) -> char,
    {
        let lower = self.lower();
        let upper = self.upper();

        for y in lower.y..=upper.y {
            for x in lower.x..=upper.x {
                let p = Point::new(x, y);
                let c = f(&p, self.get(&p));
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn draw_existing(&self) {
        self.draw(|_, s| match s {
            Some(_) => '#',
            None => '.',
        });
    }
}
