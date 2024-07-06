use std::collections::HashMap;

use aoc2015::input;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn walk_grid(directions: impl Iterator<Item = char>, visits: &mut HashMap<Coordinate, i32>) {
    let mut current_coordinate = Coordinate { x: 0, y: 0 };

    visits.insert(current_coordinate, 2);

    let mut deliver = |direction: char| {
        match direction {
            '^' => current_coordinate.y += 1,
            'v' => current_coordinate.y -= 1,
            '<' => current_coordinate.x -= 1,
            _ => current_coordinate.x += 1,
        }

        visits
            .entry(current_coordinate)
            .and_modify(|num_visits| *num_visits += 1)
            .or_insert(1);
    };

    directions.for_each(&mut deliver);
}

fn main() {
    let input = input("./inputs/day3.txt");

    let directions = input.lines().next().unwrap();
    let mut visits: HashMap<Coordinate, i32> = HashMap::new();

    walk_grid(directions.chars().step_by(2), &mut visits);
    walk_grid(directions.chars().skip(1).step_by(2), &mut visits);

    println!("{}", visits.len());
}
