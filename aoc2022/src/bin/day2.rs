use std::fs;
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let path = Path::new(r#"inputs/day2.txt"#);
    let input =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("{} could not be read", path.to_str().unwrap()));
    static WIN_PTS: i32 = 6;
    static DRAW_PTS: i32 = 3;
    static LOSE_PTS: i32 = 0;

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Play {
        Rock,
        Paper,
        Scissors,
    }

    impl Play {
        fn check(&self, other: &Self) -> i32 {
            if self == other {
                return DRAW_PTS;
            }

            if self == &Play::Rock && other == &Play::Paper {
                return LOSE_PTS;
            }

            if self == &Play::Scissors && other == &Play::Rock {
                return LOSE_PTS;
            }

            if self == &Play::Paper && other == &Play::Scissors {
                return LOSE_PTS;
            }

            WIN_PTS
        }

        fn move_pts(&self) -> i32 {
            match self {
                Play::Rock => 1,
                Play::Paper => 2,
                Play::Scissors => 3,
            }
        }
    }

    fn convert_opponent(m: &str) -> Play {
        match m {
            "A" => Play::Rock,
            "B" => Play::Paper,
            _ => Play::Scissors,
        }
    }
    fn convert_you(o: &Play, y: &str) -> Play {
        if y == "X" {
            match o {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            }
        } else if y == "Y" {
            o.clone()
        } else {
            match o {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            }
        }
    }

    fn play(opponent: &str, you: &str) -> i32 {
        let opponent = convert_opponent(opponent);
        let you = convert_you(&opponent, you);
        you.check(&opponent) + you.move_pts()
    }

    let res = input.lines().fold(0, |acc, line| {
        let chars = line.split(' ').collect::<Vec<&str>>();
        let opponent = chars[0];
        let you = chars[1];
        acc + play(opponent, you)
    });
    println!("{:?}", res);
}

fn part_one() {
    let path = Path::new(r#"inputs/day2.txt"#);
    let input =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("{} could not be read", path.to_str().unwrap()));
    static WIN_PTS: i32 = 6;
    static DRAW_PTS: i32 = 3;
    static LOSE_PTS: i32 = 0;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Play {
        Rock,
        Paper,
        Scissors,
    }

    impl Play {
        fn check(&self, other: &Self) -> i32 {
            if self == other {
                return DRAW_PTS;
            }

            if self == &Play::Rock && other == &Play::Paper {
                return LOSE_PTS;
            }

            if self == &Play::Scissors && other == &Play::Rock {
                return LOSE_PTS;
            }

            if self == &Play::Paper && other == &Play::Scissors {
                return LOSE_PTS;
            }

            WIN_PTS
        }

        fn move_pts(&self) -> i32 {
            match self {
                Play::Rock => 1,
                Play::Paper => 2,
                Play::Scissors => 3,
            }
        }
    }

    fn convert(m: &str) -> Play {
        match m {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            _ => Play::Scissors,
        }
    }

    fn play(opponent: &str, you: &str) -> i32 {
        let opponent = convert(opponent);
        let you = convert(you);
        you.check(&opponent) + you.move_pts()
    }

    let res = input.lines().fold(0, |acc, line| {
        let chars = line.split(' ').collect::<Vec<&str>>();
        let opponent = chars[0];
        let you = chars[1];
        acc + play(opponent, you)
    });
    println!("{:?}", res);
}
