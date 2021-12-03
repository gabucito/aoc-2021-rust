use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_3/input.txt") {
        // to_bit(&contents)
        println!("part 1: {:?}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

fn part1(contents: &str) -> isize {
    let rows = contents.lines().count();
    let half = rows / 2;
    let columns = contents
        .lines()
        .take(1)
        .next()
        .map(|l| return l.chars().count())
        .unwrap();
    let mut vec: Vec<u32> = vec![0; columns];
    for x in contents.lines().map(|l| {
        return l
            .chars()
            .map(|d| d.to_digit(2).unwrap())
            .collect::<Vec<u32>>();
    }) {
        let length = x.len();
        for i in 0..length {
            vec[i] += x[i]
        }
    }

    let gama = vec
        .iter()
        .map(|&i| if i > half as u32 { "1" } else { "0" })
        .collect::<String>();
    let epsilon = vec
        .iter()
        .map(|&i| if i > half as u32 { "0" } else { "1" })
        .collect::<String>();
    let gamaval = isize::from_str_radix(&gama, 2).unwrap();
    let epsilonval = isize::from_str_radix(&epsilon, 2).unwrap();
    gamaval * epsilonval

    // let half = contents.lines().count() / 2;
    // contents.lines().map(|l| return l.chars().map(|d| d.to_digit(2).unwrap()).collect::<Vec<u32>>())
    //     .fold([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |vec, next| next.map(|d| d))
}

pub fn part2(contents: &str) -> i32 {
    let rows = contents.lines().count();
    let half = rows / 2;
    let columns = contents
        .lines()
        .take(1)
        .next()
        .map(|l| return l.chars().count())
        .unwrap();
    let mut vec: Vec<u32> = vec![0; columns];
    for x in contents.lines().map(|l| {
        return l
            .chars()
            .map(|d| d.to_digit(2).unwrap())
            .collect::<Vec<u32>>();
    }) {
        let length = x.len();
        for i in 0..length {
            vec[i] += x[i]
        }
    }

    let mut filtered_oxygen: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| {
            return l
                .chars()
                .map(|d| d.to_digit(2).unwrap())
                .collect::<Vec<u32>>();
        })
        .collect();
    // println!("{:?}", filtered);
    for pos in (0..columns).cycle() {
        let rows = filtered_oxygen.iter().count();
        let half = rows / 2;
        let common_bit = get_common_bit_oxygen(&filtered_oxygen, pos, half as u32);
        filtered_oxygen = filtered_oxygen
            .into_iter()
            .filter(|line| line[pos] == common_bit)
            .collect::<Vec<Vec<u32>>>();
        if filtered_oxygen.len() == 1 {
            break;
        }
    }
    
    let mut filtered_co2: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| {
            return l
                .chars()
                .map(|d| d.to_digit(2).unwrap())
                .collect::<Vec<u32>>();
        })
        .collect();
    // println!("{:?}", filtered);
    for pos in (0..columns).cycle() {
        let rows = filtered_co2.iter().count();
        let half = rows / 2;
        let common_bit = get_common_bit_co2(&filtered_co2, pos, half as u32);
        filtered_co2 = filtered_co2
            .into_iter()
            .filter(|line| line[pos] == common_bit)
            .collect::<Vec<Vec<u32>>>();
        if filtered_co2.len() == 1 {
            break;
        }
    }

    let oxygen_rating_str = filtered_oxygen.iter().take(1).next().map(|v| v.iter().map(|d| d.to_string()).collect::<String>()).unwrap();
    let co2_rating_str = filtered_co2.iter().take(1).next().map(|v| v.iter().map(|d| d.to_string()).collect::<String>()).unwrap();
    let oxygen_rating = isize::from_str_radix(&oxygen_rating_str, 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_rating_str, 2).unwrap();
    println!("{} {}", oxygen_rating, co2_rating);
    (oxygen_rating * co2_rating) as i32
}

fn get_common_bit_oxygen(vec: &Vec<Vec<u32>>, pos: usize, half: u32) -> u32 {
    let column = vec.iter().map(|line| line[pos]).collect::<Vec<u32>>();
    let one_count = column.iter().filter(|&&digit| digit == 1).count() as u32;
    println!("{} {}", one_count, pos);
    if one_count >= half {
        1
    } else {
        0
    }
}

fn get_common_bit_co2(vec: &Vec<Vec<u32>>, pos: usize, half: u32) -> u32 {
    let column = vec.iter().map(|line| line[pos]).collect::<Vec<u32>>();
    let zero_count = column.iter().filter(|&&digit| digit == 0).count() as u32;
    if zero_count <= half {
        0
    } else {
        1
    }
}
