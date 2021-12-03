use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_3/input.txt") {
        // to_bit(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
        println!("refactor 1: {}", refactor_1(&contents));
        println!("refactor 2: {}", refactor_2(&contents));
        // println!("{:?}", iter_to_vec_int(&contents).collect::<Vec<Vec<u32>>>());
    }
}

pub fn refactor_1(contents: &str) -> u32 {
    let rows = contents.lines().count();
    let half = rows / 2;
    let columns = contents
        .lines()
        .take(1)
        .next()
        .map(|l| return l.chars().count())
        .unwrap();

    // Separate the input txt into lines
    // Map the chars into digits (0 and 1)
    // Add the digits into a vec (by folding/reduce) with initial values of 0's
    // collect the vec which will contain the total number of 1's
    let sum_of_ones = contents
        .lines()
        .fold(vec![0; columns], |vec_of_bits, line| {
            line.chars()
                .map(|d| d.to_digit(2).unwrap())
                .zip(vec_of_bits.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        });

    // get gama and epsilon as String of binary 0 and 1
    let gama = sum_of_ones
        .iter()
        .map(|&i| if i > half as u32 { "1" } else { "0" })
        .collect::<String>();
    let epsilon = sum_of_ones
        .iter()
        .map(|&i| if i > half as u32 { "0" } else { "1" })
        .collect::<String>();

    // Convert the binary string into binary and then into an integer (unsigned)
    let gamaval = u32::from_str_radix(&gama, 2).unwrap();
    let epsilonval = u32::from_str_radix(&epsilon, 2).unwrap();

    // Multiply the gama and epsilon integers
    gamaval * epsilonval
}

pub fn refactor_2(contents: &str) -> u32 {
    let binary_vec = into_binary_vec(contents);
    let oxygen_rating_val = oxygen_rating(binary_vec.clone(), 0);
    let co2_rating_val = co2_rating(binary_vec.clone(), 0);
    oxygen_rating_val * co2_rating_val
}

pub fn oxygen_rating(binary_vec: Vec<Vec<u32>>, col_position: usize) -> u32 {
    let columns = binary_vec[0].len();
    let rows = binary_vec.len();
    let half = rows / 2;
    let sum_of_ones = binary_vec
        .iter()
        .fold(vec![0; columns], |vec_of_bits, line| {
            line.iter()
                .zip(vec_of_bits.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        });
    let most_common_bit = get_most_common_bit(sum_of_ones, col_position, half as u32);
    let filtered: Vec<Vec<u32>> = binary_vec
        .into_iter()
        .filter(|line| line[col_position] == most_common_bit)
        .collect();
    if filtered.len() == 1 {
        let str: String = filtered[0].iter().map(|d| d.to_string()).collect();

        println!("oxygen_rating_str {} ", str);
        return u32::from_str_radix(&str, 2).unwrap();
    }
    oxygen_rating(filtered, col_position + 1)
}

pub fn co2_rating(binary_vec: Vec<Vec<u32>>, col_position: usize) -> u32 {
    let columns = binary_vec[0].len();
    let rows = binary_vec.len();
    let half = rows / 2;
    let sum_of_ones = binary_vec
        .iter()
        .fold(vec![0; columns], |vec_of_bits, line| {
            line.iter()
                .zip(vec_of_bits.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        });
    let most_common_bit = get_least_common_bit(sum_of_ones, col_position, half as u32);
    let filtered: Vec<Vec<u32>> = binary_vec
        .into_iter()
        .filter(|line| line[col_position] == most_common_bit)
        .collect();
    if filtered.len() == 1 {
        let str: String = filtered[0].iter().map(|d| d.to_string()).collect();
        println!("co2_rating_str {} ", str);
        return u32::from_str_radix(&str, 2).unwrap();
    }
    co2_rating(filtered, col_position + 1)
}

pub fn into_binary_vec(contents: &str) -> Vec<Vec<u32>> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|d| d.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

pub fn get_most_common_bit(sum_of_ones: Vec<u32>, col_position: usize, half: u32) -> u32 {
    let total_ones_in_first_column = sum_of_ones[col_position];
    match total_ones_in_first_column >= half {
        true => 1,
        false => 0,
    }
}

pub fn get_least_common_bit(sum_of_ones: Vec<u32>, col_position: usize, half: u32) -> u32 {
    let total_ones_in_first_column = sum_of_ones[col_position];
    match total_ones_in_first_column >= half {
        true => 0, // If 1 is more common return 0 (least common)
        false => 1,
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

    let oxygen_rating_str = filtered_oxygen
        .iter()
        .take(1)
        .next()
        .map(|v| v.iter().map(|d| d.to_string()).collect::<String>())
        .unwrap();
    let co2_rating_str = filtered_co2
        .iter()
        .take(1)
        .next()
        .map(|v| v.iter().map(|d| d.to_string()).collect::<String>())
        .unwrap();
    println!("{} | {}", oxygen_rating_str, co2_rating_str);
    let oxygen_rating = isize::from_str_radix(&oxygen_rating_str, 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_rating_str, 2).unwrap();
    (oxygen_rating * co2_rating) as i32
}

fn get_common_bit_oxygen(vec: &Vec<Vec<u32>>, pos: usize, half: u32) -> u32 {
    let column = vec.iter().map(|line| line[pos]).collect::<Vec<u32>>();
    let one_count = column.iter().filter(|&&digit| digit == 1).count() as u32;
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