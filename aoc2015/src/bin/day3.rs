use std::{self, collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("./inputs/day3.txt");
    let input = read_to_string(path).expect("File could not be read");

    #[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
    struct Coordinate {
        x: i32,
        y: i32,
    }

    let directions = input.lines().next().unwrap();
    let mut visits: HashMap<Coordinate, i32> = HashMap::new();
    let mut current_coordinate = Coordinate { x: 0, y: 0 };

    visits.insert(current_coordinate, 1);

    for direction in directions.chars().step_by(2) {
        println!("adam: {:?}", direction);
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
    let robot_directions = input.lines().next().unwrap();
    let mut robot_current_coordinate = Coordinate { x: 0, y: 0 };

    for robot_direction in robot_directions.chars().skip(1).step_by(2) {
        println!("robot: {:?}", robot_direction);
        match robot_direction {
            '^' => robot_current_coordinate.y += 1,
            'v' => robot_current_coordinate.y -= 1,
            '<' => robot_current_coordinate.x -= 1,
            _ => robot_current_coordinate.x += 1,
        }

        visits
            .entry(robot_current_coordinate)
            .and_modify(|num_visits| *num_visits += 1)
            .or_insert(1);
    }

    println!("{:?}", visits.len());
}
