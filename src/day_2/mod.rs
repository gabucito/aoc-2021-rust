use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_2/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

// count the number of times a depth measurement increases from the previous measurement
pub fn part1(contents: &str) -> i32 {
    let tuple = contents
        .lines()
        .map(|l| {
            let mut vec = l.split(" ");
            let direction = vec.next().unwrap();
            let num = vec.next().unwrap().parse::<i32>().unwrap();
            (direction, num)
        })
        .fold((0, 0), |(mut x, mut y), (direction, num)| {
            match direction {
                "forward" => x += num,
                "down" => y += num,
                "up" => y -= num,
                _ => {}
            }
            (x, y)
        });
    tuple.0 * tuple.1
}

//count the number of times the sum of measurements in this sliding window increases
pub fn part2(contents: &str) -> i64 {
    let tuple = contents
        .lines()
        .map(|l| {
            let mut vec = l.split(" ");
            let direction = vec.next().unwrap();
            let num = vec.next().unwrap().parse::<i64>().unwrap();
            (direction, num)
        })
        .fold((0, 0, 0), |(mut x, mut y, mut aim), (direction, num)| {
            match direction {
                "forward" => {
                    x += num;
                    y += num * aim;
                }
                "down" => aim += num,
                "up" => aim -= num,
                _ => {}
            }
            (x, y, aim)
        });
    tuple.0 * tuple.1
}
