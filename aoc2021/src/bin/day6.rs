use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Lanternfish {
    days_until: u8,
}

impl Lanternfish {
    fn from(days: &str) -> Self {
        let d = days.parse::<u8>().unwrap();
        Self { days_until: d }
    }

    fn handle(&mut self) -> Option<Self> {
        if self.days_until == 0 {
            self.days_until = 6;
            Some(Self { days_until: 8 })
        } else {
            self.days_until -= 1;
            None
        }
    }
}

fn main() {
    let path = Path::new("input.txt");
    let input = read_to_string(path).expect("File could not be read");

    let i: Vec<&str> = input.split(',').collect();
    let mut fish: Vec<Lanternfish> = i.iter().map(|f| Lanternfish::from(f.trim())).collect();
    const DAYS: usize = 256;

    for day in 0..DAYS {
        println!("{:?} {:?}", day, fish.len());
        for i in 0..fish.len() {
            let new_fish = fish.get_mut(i).unwrap().handle();

            if let Some(new_fish) = new_fish {
                fish.push(new_fish);
            }
        }
    }

    println!("{:?}", fish.len());
}
