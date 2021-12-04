use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_3/input.txt") {
        println!("part 1: {}", part_1(&contents));
        println!("part 2: {}", part_2(&contents));
    }
}

pub fn part_1(contents: &str) -> u32 {
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

pub fn part_2(contents: &str) -> u32 {
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
