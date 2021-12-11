use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_11/input.txt") {
        // let vec: Vec<u32> = contents
        //     .lines()
        //     .flat_map(|line| line.chars())
        //     .filter_map(|c| c.to_digit(10))
        //     .collect();

        println!("{}", part1(&contents, 100));
        println!("{}", part2(&contents));
    }
}

fn part1(contents: &str, steps: u16) -> u16 {
    let mut matrix = Matrix::new(contents);
    let mut explotions = 0;
    for _ in 0..steps {
        explotions += matrix.step();
    }
    explotions
}

fn part2(contents: &str) -> u32 {
    let mut matrix = Matrix::new(contents);
    for step in 1..1000 {
        matrix.step();
        let total = matrix.data.iter().sum::<u8>();
        if total == 0 {
            return step;
        }
    }
    0
}

#[derive(Debug)]
struct Matrix {
    data: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl Matrix {
    fn new(contents: &str) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        let data = contents
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                rows = y + 1;
                columns = line.len();
                line.chars()
            })
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Matrix {
            data,
            rows,
            columns,
        }
    }

    fn step(&mut self) -> u16 {
        self.add_to_all();
        self.check_explotions()
    }

    fn add_to_all(&mut self) {
        for x in self.data.iter_mut(){
            *x += 1        
        }
    }

    fn check_explotions(&mut self) -> u16 {
        let mut explotions = 0;
        for index in 0..self.data.len() {
            let mut num = self.data[index];
            if num > 9 {
                explotions += 1;
                self.data[index] = 0;
                let (x, y) = self.get_x_y_from_index(index);
                self.add_from_explotion(x as isize, y as isize);
            }
        }
        if explotions > 0 {
            explotions += self.check_explotions();
        }
        explotions
    }

    fn add_from_explotion(&mut self, x: isize, y: isize) {
        self.get_top_left_value(x, y);
        self.get_top_value(x, y);
        self.get_top_right_value(x, y);
        self.get_left_value(x, y);
        self.get_right_value(x, y);
        self.get_bottom_left_value(x, y);
        self.get_bottom_value(x, y);
        self.get_bottom_right_value(x, y);
    }

    fn get_value_from_x_y(&self, x: isize, y: isize) -> u8 {
        match x >= 0 && y >= 0 && x < self.columns as isize && y < self.rows as isize {
            true => {
                let index = y as usize * self.columns + x as usize;
                return self.data[index];
            }
            false => 0,
        }
    }

    fn get_x_y_from_index(&self, index: usize) -> (usize, usize) {
        let y = index / self.columns;
        let x = index % self.columns;
        (x, y)
    }

    fn get_index_from_x_y(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    fn get_top_left_value(&mut self, x: isize, y: isize) {
        let x = x - 1;
        let y = y - 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_top_value(&mut self, x: isize, y: isize) {
        let y = y - 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_top_right_value(&mut self, x: isize, y: isize) {
        let x = x + 1;
        let y = y - 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_left_value(&mut self, x: isize, y: isize) {
        let x = x - 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_right_value(&mut self, x: isize, y: isize) {
        let x = x + 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_bottom_left_value(&mut self, x: isize, y: isize) {
        let x = x - 1;
        let y = y + 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_bottom_value(&mut self, x: isize, y: isize) {
        let y = y + 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }

    fn get_bottom_right_value(&mut self, x: isize, y: isize) {
        let x = x + 1;
        let y = y + 1;
        if self.get_value_from_x_y(x, y) != 0 {
            let index = self.get_index_from_x_y(x as usize, y as usize);
            self.data[index] += 1;
        };
    }
}

#[cfg(test)]
mod test {
    use crate::day_11::{part1, part2};

    #[test]
    fn test() {
        assert_eq!(
            204,
            part1(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
                10
            )
        );
        
        assert_eq!(
            1656,
            part1(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
                100
            )
        );

        
        assert_eq!(
            195,
            part2("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")
        )
    }
}
