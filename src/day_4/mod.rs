use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_4/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<u32>>,
    won: bool,
    last_number: u32,
}

impl Board {
    fn fill_number(&mut self, number: u32) {
        if !self.won {
            for rows in self.numbers.iter_mut() {
                for num in rows.iter_mut() {
                    if num == &number {
                        *num = 0;
                    }
                }
            }
        }
        // let index =self.numbers.iter().position(|&x| x == num).unwrap();
        // self.numbers.remove(index);
    }

    fn check_row(&self) -> bool {
        for row in &self.numbers {
            if row.iter().sum::<u32>() == 0 {
                return true;
            }
        }
        false
    }

    fn check_column(&self) -> bool {
        let len = self.numbers[0].len();
        let mut iters: Vec<_> = self
            .numbers
            .clone()
            .into_iter()
            .map(|n| n.into_iter())
            .collect();
        let transposed: Vec<Vec<u32>> = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        for row in &transposed {
            if row.iter().sum::<u32>() == 0 {
                return true;
            }
        }
        false
    }

    fn get_sum(&self) -> u32 {
        self.numbers.iter().fold(0, |total, line| {
            let line_total: u32 = line.iter().sum();
            total + line_total
        }) * self.last_number
    }

    fn wins(&mut self, last_number: u32) {
        self.won = true;
        self.last_number = last_number;
    }
}

pub fn part1(contents: &str) -> u32 {
    let numbers = contents
        .lines()
        .take(1)
        .flat_map(|str| str.split(",").map(|c| c.parse::<u32>().unwrap()))
        .collect::<Vec<u32>>();

    let boards_string = contents.split("\n\n").skip(1).collect::<Vec<&str>>();
    let mut boards: Vec<Board> = Vec::new();
    for board in boards_string {
        boards.push(Board {
            numbers: board
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
            won: false,
            last_number: 0,
        });
    }

    for n in numbers {
        for board in boards.iter_mut() {
            board.fill_number(n);
            if board.check_row() || board.check_column() {
                board.wins(n);
                return board.get_sum();
            }
        }
    }

    0
}

pub fn part2(contents: &str) -> u32 {
    let numbers = contents
        .lines()
        .take(1)
        .flat_map(|str| str.split(",").map(|c| c.parse::<u32>().unwrap()))
        .collect::<Vec<u32>>();

    let boards_string = contents.split("\n\n").skip(1).collect::<Vec<&str>>();
    let mut boards: Vec<Board> = Vec::new();
    for board in boards_string {
        boards.push(Board {
            numbers: board
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
            won: false,
            last_number: 0,
        });
    }

    let mut last_board: Option<Board> = None;
    for n in numbers {
        for board in boards.iter_mut() {
            board.fill_number(n);
            if !board.won && (board.check_row() || board.check_column()) {
                board.wins(n);
                last_board = Some(board.clone());         
            }
        }
    }

    if let Some(board) = last_board {
        let total = board.get_sum();
        return board.get_sum()
    }

    0
}
