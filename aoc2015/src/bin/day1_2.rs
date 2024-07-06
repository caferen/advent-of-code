use aoc2015::input;

const INCR: i32 = 1;

fn main() {
    let input = input("./inputs/day1.txt");

    let mut floor = 0;
    let mut counter = 0;

    let mut make_move = |c: char| {
        match c {
            ')' => floor -= INCR,
            '(' => floor += INCR,
            _ => println!("Invalid char"),
        }

        counter += 1;

        if floor == -1 {
            return true;
        }

        false
    };

    for c in input.chars() {
        let did_hit_basement = make_move(c);
        if did_hit_basement {
            break;
        }
    }
    println!("{counter}");
}
