use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Galaxy,
}
struct GalaxyMap {
    map: Vec<Vec<Tile>>,
}

impl GalaxyMap {
    pub fn from_str(input: &str) -> Self {
        GalaxyMap {
            map: input
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Tile::Empty,
                            '#' => Tile::Galaxy,
                            _ => unreachable!(),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
    pub fn get_empty_rows(&self) -> Vec<u32> {
        let mut rows = vec![];
        let len = self.map.len() - 1;
        for i in 0..len {
            let row = self.map.get(i).unwrap();
            if row.iter().all(|t| match t {
                Tile::Empty => true,
                Tile::Galaxy => false,
            }) {
                rows.push(i as u32);
            }
        }
        rows
    }
    pub fn get_empty_cols(&self) -> Vec<u32> {
        let mut cols = vec![];
        let len = self.map.first().unwrap().len();
        for i in 0..len {
            if self.map.iter().all(|r| match r.get(i).unwrap() {
                Tile::Empty => true,
                Tile::Galaxy => false,
            }) {
                cols.push(i as u32);
            }
        }
        cols
    }

    pub fn expand(&mut self) {
        // Lines
        let len = self.map.len() - 1;
        for i in 0..len {
            let i = (len - 1) - i;
            let row = self.map.get(i).unwrap();
            if row.iter().all(|t| match t {
                Tile::Empty => true,
                Tile::Galaxy => false,
            }) {
                self.map.insert(i, row.clone())
            }
        }
        // Columns
        let len = self.map.first().unwrap().len();
        for i in 0..len {
            let i = (len - 1) - i;
            if self.map.iter().all(|r| match r.get(i).unwrap() {
                Tile::Empty => true,
                Tile::Galaxy => false,
            }) {
                for row in self.map.iter_mut() {
                    row.insert(i, Tile::Empty)
                }
            }
        }
    }
    pub fn print(&self) {
        for line in &self.map {
            for c in line {
                match c {
                    Tile::Empty => print!("."),
                    Tile::Galaxy => print!("#"),
                }
            }
            println!();
        }
    }

    pub fn part1(&self) -> i32 {
        let mut galaxy_locations = vec![];
        for (r, row) in self.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if tile == &Tile::Galaxy {
                    galaxy_locations.push((r, c));
                }
            }
        }
        let all_pairs = galaxy_locations.iter().tuple_combinations().collect_vec();
        let r = all_pairs.iter().map(|((r1, c1), (r2, c2))| {
            (*r1 as i32 - *r2 as i32).abs() + (*c1 as i32 - *c2 as i32).abs()
        });
        r.sum()
    }

    pub fn part2(&self) -> i64 {
        let mut galaxy_locations = vec![];
        for (r, row) in self.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if tile == &Tile::Galaxy {
                    galaxy_locations.push((r, c));
                }
            }
        }
        let e_rows = self.get_empty_rows();
        let e_cols = self.get_empty_cols();
        let all_pairs = galaxy_locations.iter().tuple_combinations().collect_vec();
        let r = all_pairs.iter().map(|((r1, c1), (r2, c2))| {
            let mut empties = 0;
            empties += e_rows
                .iter()
                .filter(|r| {
                    let r = &(**r as usize);
                    (r1 > r && r2 < r) || (r2 > r && r1 < r)
                })
                .count();
            empties += e_cols
                .iter()
                .filter(|c| {
                    let c = &(**c as usize);
                    (c1 > c && c2 < c) || (c2 > c && c1 < c)
                })
                .count();

            ((*r1 as i64 - *r2 as i64).abs() + (*c1 as i64 - *c2 as i64).abs())
                + ((1000000 - 1) * (empties as i64))
        });
        r.sum()
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    let mut map = GalaxyMap::from_str(input);
    map.expand();
    map.print();
    Some(map.part1())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut map = GalaxyMap::from_str(input);
    Some(map.part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}