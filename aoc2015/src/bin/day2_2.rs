use aoc2015::input;

fn main() {
    let input = input("./inputs/day2.txt");
    let lines = input.lines();

    let total = lines.into_iter().fold(0, |acc, line| {
        let side_dimensions = line
            .split('x')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let sum_all: i32 = side_dimensions.iter().sum();

        let dimensions: Vec<i32> = side_dimensions.iter().map(|d| 2 * (sum_all - d)).collect();

        let minimum = dimensions.iter().min().unwrap();

        let volume = side_dimensions.iter().product::<i32>();

        acc + volume + minimum
    });

    println!("{total}");
}
