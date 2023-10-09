use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new(r#"inputs/day2.txt"#);
    let input =
        fs::read_to_string(path).expect(&format!("{} could not be read", path.to_str().unwrap()));
    static WIN_PTS: i32 = 6;
    static DRAW_PTS: i32 = 3;
    static LOSE_PTS: i32 = 0;

    let mut res = 0;

    fn move_to_pts(m: &str) -> i32 {
        match m {
            "A" => 1,
            "B" => 2,
            _ => 3,
        }
    }

    fn lose(opponent: &str) -> i32 {
        match opponent {
            "A" => LOSE_PTS + move_to_pts("C"),
            "B" => LOSE_PTS + move_to_pts("A"),
            _ => LOSE_PTS + move_to_pts("B"),
        }
    }

    fn win(opponent: &str) -> i32 {
        match opponent {
            "A" => WIN_PTS + move_to_pts("B"),
            "B" => WIN_PTS + move_to_pts("C"),
            _ => WIN_PTS + move_to_pts("A"),
        }
    }

    fn draw(opponent: &str) -> i32 {
        match opponent {
            "A" => DRAW_PTS + move_to_pts("A"),
            "B" => DRAW_PTS + move_to_pts("B"),
            _ => DRAW_PTS + move_to_pts("C"),
        }
    }

    fn goal_and_opponent_to_points(goal: &str, opponent: &str) -> i32 {
        match goal {
            "X" => lose(opponent),
            "Y" => draw(opponent),
            _ => win(opponent),
        }
    }

    for i in input.lines() {
        let chars = i.split(' ').collect::<Vec<&str>>();
        let opponent = chars[0];
        let goal = chars[1];
        res += goal_and_opponent_to_points(goal, opponent);
    }

    println!("{:?}", res);
}
