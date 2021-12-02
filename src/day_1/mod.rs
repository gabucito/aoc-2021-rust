use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_1/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

// count the number of times a depth measurement increases from the previous measurement
pub fn part1(contents: &str) -> i32 {
    let tuple = contents.lines().map(|l| l.parse::<i32>().unwrap()).fold((0, 0), |(last, mut increment), next| {
        if next > last {
            increment += 1;
        }
        (next, increment)
    });
    tuple.1 - 1
}

//count the number of times the sum of measurements in this sliding window increases
pub fn part2(contents: &str) -> i32 {
    let vec: Vec<i32> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let tuple = vec.windows(3).fold((0, 0), |(last, mut increment), next| {
        let sum = next.iter().sum();
        if sum > last {
            increment+= 1;
        }
        (sum, increment)
    });
    tuple.1 - 1
}