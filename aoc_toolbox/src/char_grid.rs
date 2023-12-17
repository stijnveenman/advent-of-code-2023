use std::collections::HashMap;

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
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(y, c)| Some((Point::new(x as isize, y as isize), parse_char(c)?)))
            })
            .flatten()
            .collect();

        let lower = Point::new(0, 0);
        let upper = Point::new(
            s.lines().next().unwrap().len() as isize - 1,
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
}
