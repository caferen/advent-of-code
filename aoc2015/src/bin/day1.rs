use std::fs;
fn main() {
    let inputs = fs::read_to_string("./inputs/day1.txt").expect("File could not be read");

    let mut floor = 0;
    let mut counter = 0;
    const INCR: i32 = 1;

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

    for c in inputs.chars() {
        let did_hit_basement = make_move(c);
        if did_hit_basement {
            break;
        }
    }
    println!("{}", counter);
}
