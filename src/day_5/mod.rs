use std::collections::HashMap;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_5/input.txt") {
        println!("part 1: {}", part1(&contents));
        // println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> i32 {
    let vector_from_to = contents
        .lines()
        .map(|line| line.split("->"))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect::<Vec<(&str, &str)>>();

    let hash = vector_from_to
        .iter()
        .fold(HashMap::new(), |mut hash, (from, to)| {
            let mut left = from.split(',');
            let mut from_x = left.next().unwrap().trim().parse::<i32>().unwrap();
            let mut from_y = left.next().unwrap().trim().parse::<i32>().unwrap();

            let mut right = to.split(',');
            let mut to_x = right.next().unwrap().trim().parse::<i32>().unwrap();
            let mut to_y = right.next().unwrap().trim().parse::<i32>().unwrap();

            match (from_x == to_x, from_y == to_y) {
                (true, false) => {
                    // draw horizontal
                    if from_y > to_y {
                        std::mem::swap(&mut from_y, &mut to_y);
                    }
                    for y in from_y..=to_y {
                        let counter = hash.entry((from_x, y)).or_insert(0);
                        *counter += 1;
                    }
                }
                (false, true) => {
                    // draw vertical
                    if from_x > to_x {
                        std::mem::swap(&mut from_x, &mut to_x);
                    }
                    for x in from_x..=to_x {
                        let counter = hash.entry((x, from_y)).or_insert(0);
                        *counter += 1;
                    }
                }
                (false, false) => {
                }
                _ => {}
            }
            hash
        });

    hash.iter().filter(|(_, &count)| count > 1).count() as i32
}
pub fn part2(contents: &str) -> i32 {
    let vector_from_to = contents
        .lines()
        .map(|line| line.split("->"))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect::<Vec<(&str, &str)>>();

    let hash = vector_from_to
        .iter()
        .fold(HashMap::new(), |mut hash, (from, to)| {
            let mut left = from.split(',');
            let mut from_x = left.next().unwrap().trim().parse::<i32>().unwrap();
            let mut from_y = left.next().unwrap().trim().parse::<i32>().unwrap();

            let mut right = to.split(',');
            let mut to_x = right.next().unwrap().trim().parse::<i32>().unwrap();
            let mut to_y = right.next().unwrap().trim().parse::<i32>().unwrap();

            match (from_x == to_x, from_y == to_y) {
                (true, false) => {
                    // draw horizontal
                    if from_y > to_y {
                        std::mem::swap(&mut from_y, &mut to_y);
                    }
                    for y in from_y..=to_y {
                        let counter = hash.entry((from_x, y)).or_insert(0);
                        *counter += 1;
                    }
                }
                (false, true) => {
                    // draw vertical
                    if from_x > to_x {
                        std::mem::swap(&mut from_x, &mut to_x);
                    }
                    for x in from_x..=to_x {
                        let counter = hash.entry((x, from_y)).or_insert(0);
                        *counter += 1;
                    }
                }
                (false, false) => {
                    let x_diff = to_x - from_x;
                    let y_diff = to_y - from_y;
                    let x_slope = match x_diff.is_negative() {
                        true => -1,
                        false => 1,
                    };
                    let y_slope = match y_diff.is_negative() {
                        true => -1,
                        false => 1,
                    };
                    let mut y_index = 0;
                    let mut x_index = 0;

                    for _ in 0..=x_diff.abs() {
                        let x = from_x + x_index;
                        let y = from_y + y_index;
                        let counter = hash.entry((x, y)).or_insert(0);
                        *counter += 1;

                        x_index += x_slope;
                        y_index += y_slope;
                    }
                }
                _ => {}
            }
            hash
        });

    hash.iter().filter(|(_, &count)| count > 1).count() as i32
}

// #[cfg(test)]
// mod tests {
//     use crate::day_5::part1;

//     #[test]
//     fn diagonal() {
//         assert_eq!(
//             12,
//             part1(
//                 "0,9 -> 5,9
//         8,0 -> 0,8
//         9,4 -> 3,4
//         2,2 -> 2,1
//         7,0 -> 7,4
//         6,4 -> 2,0
//         0,9 -> 2,9
//         3,4 -> 1,4
//         0,0 -> 8,8
//         5,5 -> 8,2"
//             )
//         )
//     }
// }
