use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_7/input.txt") {
        println!("{}", part1(&contents));
        println!("{}", part2(&contents));
    }
}

fn part1(contents: &str) -> i32 {
    let mut num_vec: Vec<i32> = contents
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    num_vec.sort();
    let length = num_vec.len();
    let median = num_vec[length / 2];
    num_vec.iter().fold(0, |total, num| total + (num - median).abs())
}

fn part2(contents: &str) -> i32 {
    let num_vec: Vec<i32> = contents
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let length = num_vec.len();
    let mean: i32 = (num_vec.iter().sum::<i32>() as f64 / length as f64).round() as i32;
    let mut total: i32 = i32::MAX;
    // it should be near the mean
    for m in mean-1..mean+1 {
        let t = num_vec.iter().fold(0, |total, num| {
            let n = (num - m).abs();
            let add = (n.pow(2) + n) / 2;
            total + add
        });
        if t < total {
            total = t;
        }
    }    
    total
}

#[cfg(test)]
mod tests {
    use crate::day_7::{part1, part2};
    #[test]
    fn test() {
        assert_eq!(37, part1("16,1,2,0,4,2,7,1,2,14"));
        assert_eq!(168, part2("16,1,2,0,4,2,7,1,2,14"));
        // assert_eq!(168, part2("16,1,2,0,4,2,7,1,2,14"));

        // assert_eq!(
        //     26984457539,
        //     part2("3,4,3,1,2", 256)
        // );
    }
}
