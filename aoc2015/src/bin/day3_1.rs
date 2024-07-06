use std::collections::HashMap;

use aoc2015::input;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let input = input("./inputs/day3.txt");

    let directions = input.lines().next().unwrap();
    let mut visits: HashMap<Coordinate, i32> = HashMap::new();
    let mut current_coordinate = Coordinate { x: 0, y: 0 };

    visits.insert(current_coordinate, 1);

    for direction in directions.chars() {
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
    }

    println!("{}", visits.len());
}
