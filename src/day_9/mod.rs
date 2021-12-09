use std::collections::{HashMap, HashSet};

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_9/input.txt") {
        println!("{}", part1(&contents));
        println!("{}", part2(&contents));
    }
}

fn part1(contents: &str) -> u32 {
    let matrix = Matrix::new(contents);
    matrix.sum_low_points()
}
fn part2(contents: &str) -> u32 {
    let matrix = Matrix::new(contents);
    matrix.get_basins() as u32
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

    fn get_value_from_x_y(&self, x: usize, y: usize) -> u8 {
        if x < self.columns && y < self.rows {
            let index = y * self.columns + x;
            return self.data[index];
        }
        9
    }

    fn get_x_y_from_index(&self, index: usize) -> (usize, usize) {
        let y = index / self.columns;
        let x = index % self.columns;
        (x, y)
    }

    fn get_index_from_x_y(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    fn check_lower_than_adjacent(&self, index: usize) -> bool {
        let (x, y) = self.get_x_y_from_index(index);
        let current_height = self.get_value_from_x_y(x, y);

        let top_height = if y > 0 {
            self.get_value_from_x_y(x, y - 1)
        } else {
            9
        };
        let bottom_height = self.get_value_from_x_y(x, y + 1);
        let left_height = if x > 0 {
            self.get_value_from_x_y(x - 1, y)
        } else {
            9
        };

        let right_height = self.get_value_from_x_y(x + 1, y);

        current_height < top_height
            && current_height < bottom_height
            && current_height < left_height
            && current_height < right_height
    }

    fn get_low_heights(&self) -> Vec<&u8> {
        self.data
            .iter()
            .enumerate()
            .filter(|&(index, _)| self.check_lower_than_adjacent(index))
            .map(|(_, height)| height)
            .collect()
    }

    fn get_low_indexes(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter(|&(index, _)| self.check_lower_than_adjacent(index))
            .map(|(index, _)| index)
            .collect()
    }

    fn sum_low_points(&self) -> u32 {
        let low_heights = self.get_low_heights();
        low_heights
            .iter()
            .fold(0, |total, &&data| total + data as u32 + 1)
    }

    fn get_basins(&self) -> usize {
        let low_indexes = self.get_low_indexes();
        let mut vec: Vec<usize> = Vec::new();
        for index in low_indexes {
            let mut set: HashSet<usize> = HashSet::new();
            self.check(index, &mut set);
            vec.push(set.len());
        }
        vec.sort();
        vec.reverse();
        vec.iter().take(3).fold(1, |total, ele| total * ele)
    }

    fn check(&self, index: usize, set: &mut HashSet<usize>) {
        set.insert(index);
        let (x, y) = self.get_x_y_from_index(index);
        let current_height = self.get_value_from_x_y(x, y);

        let top_height = if y > 0 {
            self.get_value_from_x_y(x, y - 1)
        } else {
            9
        };

        let bottom_height = self.get_value_from_x_y(x, y + 1);

        let left_height = if x > 0 {
            self.get_value_from_x_y(x - 1, y)
        } else {
            9
        };

        let right_height = self.get_value_from_x_y(x + 1, y);

        if current_height < top_height && top_height < 9 {
            let index = self.get_index_from_x_y(x, y - 1);
            self.check(index, set);
        }

        if current_height < bottom_height && bottom_height < 9 {
            let index = self.get_index_from_x_y(x, y + 1);
            self.check(index, set);
        }

        if current_height < left_height && left_height < 9 {
            let index = self.get_index_from_x_y(x - 1, y);
            self.check(index, set);
        }

        if current_height < right_height && right_height < 9 {
            let index = self.get_index_from_x_y(x + 1, y);
            self.check(index, set);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_9::{part1, part2};
    
    #[test]
    fn test() {
        assert_eq!(
            15,
            part1(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )
        );
        
        assert_eq!(
            1134,
            part2(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )
        );
    }
}
