use itertools::Itertools;

advent_of_code::solution!(3);
struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '0'..='9' => Tile::Number(c.to_digit(10).unwrap()),
                        _ => Tile::Symbol(c),
                    })
                    .collect_vec()
            })
            .collect::<Vec<_>>();
        Map { map }
    }
    pub fn collapse_numbers(&mut self) {
        for (y, row) in self.map.clone().iter().enumerate() {
            let mut working_number: (usize, Vec<u32>) = (0, Vec::new());
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Number(n) => {
                        if working_number.1.is_empty() {
                            working_number = (x, vec![*n]);
                        } else {
                            working_number.1.push(*n);
                        }
                    }
                    _ => {
                        if !working_number.1.is_empty() {
                            let whole_number =
                                working_number.1.iter().fold(0, |acc, n| acc * 10 + n);
                            let index = working_number.0;
                            for (i, _) in working_number.1.iter().enumerate() {
                                self.map[y][index + i] =
                                    Tile::CollapsedNumber((y, index), whole_number);
                            }
                            working_number = (0, Vec::new());
                        }
                    }
                }
            }
            if !working_number.1.is_empty() {
                let whole_number = working_number.1.iter().fold(0, |acc, n| acc * 10 + n);
                let index = working_number.0;
                for (i, _) in working_number.1.iter().enumerate() {
                    self.map[y][index + i] = Tile::CollapsedNumber((y, index), whole_number);
                }
            }
        }
    }
    pub fn get_numbers_around(&self, row: usize, col: usize) -> Vec<((usize, usize), u32)> {
        let mut numbers = Vec::new();
        let mut locations = Vec::new();
        for y in row - 1..=row + 1 {
            for x in col - 1..=col + 1 {
                if let Tile::CollapsedNumber(location, n) = self.map[y][x] {
                    if !locations.contains(&location) {
                        locations.push(location);
                        numbers.push((location, n));
                    }
                }
            }
        }
        numbers
    }
    pub fn get_sum(&self) -> u32 {
        let mut all_numbers = Vec::new();
        let mut all_locations = Vec::new();
        let map = self.map.clone();
        for (y, row) in map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Symbol(_) = tile {
                    let numbers = self.get_numbers_around(y, x);
                    for (location, n) in numbers {
                        if !all_locations.contains(&location) {
                            all_locations.push(location);
                            all_numbers.push(n);
                        }
                    }
                }
            }
        }
        all_numbers.iter().sum()
    }

    pub fn get_gear_ratio_sum(&self) -> u32 {
        let map = self.map.clone();
        let mut sum = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Symbol('*') = tile {
                    let numbers = self.get_numbers_around(y, x);
                    if numbers.len() == 2 {
                        let (_, n1) = numbers[0];
                        let (_, n2) = numbers[1];
                        sum += n1 * n2;
                    }
                }
            }
        }
        sum
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Number(u32),
    Empty,
    Symbol(char),
    CollapsedNumber((usize, usize), u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    map.collapse_numbers();
    Some(map.get_sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    map.collapse_numbers();
    Some(map.get_gear_ratio_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
