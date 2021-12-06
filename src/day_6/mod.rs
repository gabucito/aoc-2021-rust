use std::mem::take;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_6/input.txt") {
        // let vec: Vec<Lanternfish> = contents.split(',').map(|c| Lanternfish {
        //     days: c.parse::<u8>().unwrap()
        // }).collect();
        // let school = SchoolLanterfish {
        //     fish: vec
        // };
        println!("{}", part1(&contents, 80));
        println!("{}", part2(&contents, 256));
        // println!("part 1: {}", part1(&contents, 120));
        // println!("part 2: {}", part2(&contents));
    }
}

fn part1(contents: &str, days: u32) -> u64 {
    let vec: Vec<Lanternfish> = contents.split(',').map(|c| Lanternfish {
        days: c.parse::<u8>().unwrap()
    }).collect();
    let mut school = SchoolLanterfish {
        fish: vec
    };
    for _ in 1..=days {
        school.pass_day();
    }
    school.fish.len() as u64
}


fn part2(contents: &str, days: u32) -> u128 {
    let vec: Vec<usize> = contents.split(',').map(|c| c.parse::<usize>().unwrap()).collect();

    // lookup table
    let mut group_fish_by_days: [u128; 9] = [0; 9];
    for fish in vec {
        group_fish_by_days[fish] += 1;
    }
    
    for _ in 0..days {
        group_fish_by_days.rotate_left(1);
        // ADD 1 fish to the sixth day for every fish that got rotated to the eight day
        group_fish_by_days[6] += group_fish_by_days[8];
    }

    group_fish_by_days.iter().fold(0, |total, fish| total + fish)
}


#[derive(Debug)]
struct SchoolLanterfish {
    fish: Vec<Lanternfish>
}

impl SchoolLanterfish {
    fn add_fish(&mut self, lanternfish: Lanternfish) {
        self.fish.push(lanternfish);
    }

    fn pass_day(&mut self) {
        let length = self.fish.len();
        for i in 0..length {
            let fish = &mut self.fish[i];
            if let Some(lantern) = fish.pass_day() {
                self.add_fish(lantern);
            }
        }
    }

    fn after_days(&mut self, days: u32) {
        let starting_fish_total = self.fish.len();
        let rate = days/7;
    }
}

#[derive(Debug)]
struct Lanternfish {
    days: u8,   
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Self {
            days: 8
        }
    }

    fn pass_day(&mut self) -> Option<Lanternfish> {
        match self.days == 0 {
            true => {
                self.days = 6;
                Some(Lanternfish::new())
            },
            false => {
                self.days -= 1;
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_6::{part1, part2};


    #[test]
    fn test() {
        assert_eq!(
            5934,
            part1("3,4,3,1,2", 80)
        );

        assert_eq!(
            26984457539,
            part2("3,4,3,1,2", 256)
        );
    }
}
