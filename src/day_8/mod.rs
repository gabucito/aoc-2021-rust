use std::collections::HashMap;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_8/input.txt") {
        println!("{}", part1(&contents));
        println!("{}", part2(&contents));
    }
}

fn part1(contents: &str) -> usize {
    contents
        .lines()
        .map(|line| line.split("|"))
        .filter_map(|split| split.skip(1).next())
        .flat_map(|second| second.trim().split_whitespace())
        .filter(|c| match c.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(contents: &str) -> usize {
    // contents.lines().map(|line| get_line_number(line)).fold(0, |total, num| total + num)
    let vec = contents.lines().map(|line| get_line_number(line)).collect::<Vec<usize>>();
    println!("{:?}", vec);
    vec.iter().sum::<usize>()
}

fn get_line_number(contents: &str) -> usize {
    let mut split = contents.split('|');
    let left = split.next().unwrap().trim();
    let right = split.next().unwrap().trim();
    let mut hash: HashMap<String, u32> = HashMap::new();
    let mut left_segment: Vec<&str> = left.split_whitespace().collect();
    left_segment.sort_by(|&a, &b| a.len().cmp(&b.len()));

    left_segment.iter().for_each(|word| {
        let mut char_vec = word.chars().collect::<Vec<char>>();
        char_vec.sort();
        let str: String = char_vec.iter().collect();
        match word.len() {
            2 => hash.insert(str, 1),
            3 => hash.insert(str, 7),
            4 => hash.insert(str, 4),
            7 => hash.insert(str, 8),
            5 => {
                // can be 2, 3 or 5

                // to get 2, compare it with 4's string
                // it should have 3 more char

                // to get 3, compare it with 7's string
                // it should have 2 more char

                // to get 5, first get 2, 3 first


                let string_4 = hash.iter().find(|(_, &v)| v == 4).map(|(k, _)| k).unwrap();
                let string_7 = hash.iter().find(|(_, &v)| v == 7).map(|(k, _)| k).unwrap();
                if char_vec.iter().filter(|c| !string_4.contains(&c.to_string())).collect::<String>().len() == 3 {
                    hash.insert(str, 2)
                } else if char_vec.iter().filter(|c| !string_7.contains(&c.to_string())).collect::<String>().len() == 2 {
                    hash.insert(str, 3)
                } else {
                    hash.insert(str, 5)
                }
            }
            6 => {
                // can be 0, 6, 9

                // to get 6, compare it with 7's string
                // if there is only 4 char that is different then it is 6

                // to get 9, compare it with 4's string
                // 9 should have only 2 characters more than 4

                // to get 0, first get 6 and 9 and if it is not either then it is 0

                let string_4 = hash.iter().find(|(_, &v)| v == 4).map(|(k, _)| k).unwrap();
                let string_7 = hash.iter().find(|(_, &v)| v == 7).map(|(k, _)| k).unwrap();                
                if char_vec.iter().filter(|c| !string_7.contains(&c.to_string())).collect::<String>().len() == 4 {
                    hash.insert(str, 6)
                } else if char_vec.iter().filter(|c| !string_4.contains(&c.to_string())).collect::<String>().len() == 2 {
                    hash.insert(str, 9)
                } else {
                    hash.insert(str, 0)
                }
            }
            _ => None,
        };
    });
    println!("{:?}", hash);

    let test= right.split_whitespace().map(|word| {
        let mut char_vec = word.chars().collect::<Vec<char>>();
        char_vec.sort();
        let str: String = char_vec.iter().collect();
        println!("{}", str);
        let number = hash.get(&str).unwrap();
        number
    }).fold( 0, |total, num| total * 10 + num);
    test as usize
}

#[cfg(test)]
mod tests {
    use crate::day_8::{part1, part2};
    #[test]
    fn test() {
        assert_eq!(
            26,
            part1(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )
        );
        assert_eq!(
            61229,
            part2(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )
        );
        assert_eq!(5353, part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));
    }
}
