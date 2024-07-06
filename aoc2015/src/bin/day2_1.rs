use aoc2015::input;

fn main() {
    let input = input("./inputs/day2.txt");
    let lines = input.lines();

    let total = lines.into_iter().fold(0, |acc, line| {
        let side_dimensions = line
            .split('x')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let product: i32 = side_dimensions.iter().product();

        let areas: Vec<i32> = side_dimensions.iter().map(|d| 2 * (product / d)).collect();

        let minimum = areas.iter().min().unwrap() / 2;

        let areas_sum: i32 = areas.iter().sum();

        acc + areas_sum + minimum
    });

    println!("{total}");
}
