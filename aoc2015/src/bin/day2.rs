use std::{fs, path::Path};

fn main() {
    let path = Path::new("./inputs/day2.txt");
    let input = fs::read_to_string(path).expect("File could not be read");

    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        let side_dimensions = line
            .split('x')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        fn calculate_areas(dimensions: &[i32]) -> Vec<i32> {
            let sum_all: i32 = dimensions.iter().sum();

            dimensions.iter().map(|d| 2 * (sum_all - d)).collect()
        }

        let dimensions = calculate_areas(&side_dimensions);

        let minimum = dimensions.iter().min().unwrap();

        let volume = side_dimensions.iter().product::<i32>();

        total += volume + minimum;
    }
    println!("{}", total);
}
