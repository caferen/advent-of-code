use std::{collections::HashMap, fs, hash::Hash};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn get_vertical_length(&self) -> u64 {
        self.end.y.abs_diff(self.start.y)
    }

    pub fn get_hortizontal_length(&self) -> u64 {
        self.end.x.abs_diff(self.start.x)
    }

    pub fn limit(self) -> u64 {
        std::cmp::max(self.get_hortizontal_length(), self.get_vertical_length()) + 1
    }

    pub fn is_eligible(self) -> bool {
        self.get_hortizontal_length() == 0
            || self.get_vertical_length() == 0
            || self.get_hortizontal_length() == self.get_vertical_length()
    }

    pub fn calculate_vent_coordinates(&self) -> Vec<Point> {
        let mut vents: Vec<Point> = vec![];

        let horizontal = self.end.x - self.start.x;
        let vertical = self.end.y - self.start.y;
        let horizontal_increment = match horizontal {
            0 => 0,
            _ => horizontal.abs() / horizontal,
        };
        let vertical_increment = match vertical {
            0 => 0,
            _ => vertical.abs() / vertical,
        };

        let mut current = self.start;
        if self.is_eligible() {
            for _ in 0..self.limit() {
                vents.push(current);

                current.x += horizontal_increment;
                current.y += vertical_increment;
            }
        }
        vents
    }
}

fn count_overlapping_vents(grid: HashMap<Point, u64>) -> u64 {
    let mut ov: u64 = 0;
    for (_p, c) in grid {
        if c >= 2 {
            ov += 1;
        }
    }
    println!("{:?}", ov);
    ov
}

fn main() {
    let input = fs::read_to_string("./inputs/input.txt").expect("File could not be read!");
    let lines = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let point_pairs = lines
        .iter()
        .map(|line| {
            line.split("->")
                .filter(|spl| spl.len() > 1)
                .collect::<Vec<&str>>()
                .iter()
                .map(|point| {
                    let coords = point
                        .split(',')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.trim().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    Point::new(coords[0], coords[1])
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut vents: HashMap<Point, u64> = HashMap::new();

    for point_pair in point_pairs {
        let line = Line::new(point_pair[0], point_pair[1]);
        let vent_coordinates = line.calculate_vent_coordinates();

        for vc in vent_coordinates {
            vents.entry(vc).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    count_overlapping_vents(vents);
}
