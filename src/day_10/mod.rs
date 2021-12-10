use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_10/input.txt") {
        println!("{}", part1(&contents));
        println!("{}", part2(&contents));
    }
}

fn part1(contents: &str) -> u32 {
    let mut err_stack: Vec<char> = Vec::new();
    let mut lines_without_corrupted: Vec<&str> = Vec::new();
    'line: for line in contents.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        err_stack.push(c);
                        continue 'line;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        err_stack.push(c);
                        continue 'line;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        err_stack.push(c);
                        continue 'line;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        err_stack.push(c);
                        continue 'line;
                    }
                }
                _ => {}
            };
        }
        lines_without_corrupted.push(line);
    }
    err_stack.iter().fold(0, |total, c| match c {
        ')' => total + 3,
        ']' => total + 57,
        '}' => total + 1197,
        '>' => total + 25137,
        _ => total,
    })
}

fn part2(contents: &str) -> u64 {
    let mut skipped_total = Vec::new();
    let mut lines_without_corrupted: Vec<&str> = Vec::new();
    'line: for line in contents.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        continue 'line;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        continue 'line;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        continue 'line;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        continue 'line;
                    }
                }
                _ => {}
            };
        }
        lines_without_corrupted.push(line);
    }
    // err_stack.iter().fold(0,|total, c| match c {
    //     ')' => total + 3,
    //     ']' => total + 57,
    //     '}' => total + 1197,
    //     '>' => total + 25137,
    //     _ => total,
    // })
    for line in lines_without_corrupted {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => {}
            };
        }
        let mut err_stack: Vec<char> = Vec::new();
        while let Some(popped) = stack.pop() {
            match popped {
                '(' => err_stack.push(')'),
                '[' => err_stack.push(']'),
                '{' => err_stack.push('}'),
                '<' => err_stack.push('>'),
                _ => {}
            }
        }
        let total = err_stack.iter().fold(0, |total, c| match c {
            ')' => total * 5 + 1,
            ']' => total * 5 + 2,
            '}' => total * 5 + 3,
            '>' => total * 5 + 4,
            _ => total,
        });
        skipped_total.push(total);
    }
    let index = skipped_total.len() / 2;
    skipped_total.sort();
    skipped_total[index]
}

#[cfg(test)]
mod tests {
    use crate::day_10::{part1, part2};

    #[test]
    fn test() {
        assert_eq!(
            26397,
            part1(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            )
        );
        
        assert_eq!(
            288957,
            part2(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            )
        );
    }
}
