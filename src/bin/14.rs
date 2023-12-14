use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(14);
#[derive(Debug, Clone, Hash)]
struct Platform {
    map: Vec<Vec<char>>,
    rocks: Vec<(usize, usize)>,
}

impl Platform {
    pub fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let rocks = map
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 'O')
                    .map(move |(c, _)| (r, c))
            })
            .collect();
        Self { map, rocks }
    }

    pub fn tilt_north(&mut self) {
        loop {
            let mut stuff_changed = false;
            for r in 1..self.map.len() {
                for c in 0..self.map[r].len() {
                    let prev = self.map[r - 1][c];
                    let curr = self.map[r][c];

                    if prev == '.' && curr == 'O' {
                        stuff_changed = true;
                        self.map[r - 1][c] = 'O';
                        self.map[r][c] = '.';
                    }
                }
            }
            if !stuff_changed {
                break;
            }
        }
    }
    pub fn tilt_cycle(&mut self) {
        //North
        loop {
            let mut stuff_changed = false;
            for rock in &mut self.rocks {
                let (r, c) = *rock;
                if r > 0 && self.map[r - 1][c] == '.' {
                    self.map[r][c] = '.';
                    self.map[r - 1][c] = 'O';
                    stuff_changed = true;
                    *rock = (r - 1, c);
                }
            }
            if !stuff_changed {
                break;
            }
        }
        // West
        loop {
            let mut stuff_changed = false;
            for rock in &mut self.rocks {
                let (r, c) = *rock;
                if c > 0 && self.map[r][c - 1] == '.' {
                    self.map[r][c] = '.';
                    self.map[r][c - 1] = 'O';
                    stuff_changed = true;
                    *rock = (r, c - 1);
                }
            }
            if !stuff_changed {
                break;
            }
        }
        // South
        loop {
            let mut stuff_changed = false;
            for rock in &mut self.rocks {
                let (r, c) = *rock;
                if r < self.map.len() - 1 && self.map[r + 1][c] == '.' {
                    self.map[r][c] = '.';
                    self.map[r + 1][c] = 'O';
                    stuff_changed = true;
                    *rock = (r + 1, c);
                }
            }
            if !stuff_changed {
                break;
            }
        }
        // East
        loop {
            let mut stuff_changed = false;
            for rock in &mut self.rocks {
                let (r, c) = *rock;
                if c < self.map.len() - 1 && self.map[r][c + 1] == '.' {
                    self.map[r][c] = '.';
                    self.map[r][c + 1] = 'O';
                    stuff_changed = true;
                    *rock = (r, c + 1);
                }
            }
            if !stuff_changed {
                break;
            }
        }
    }

    pub fn get_load(&self) -> u32 {
        let mut load = 0;
        for r in 0..self.map.len() {
            for c in 0..self.map[r].len() {
                if self.map[r][c] == 'O' {
                    load += self.map.len() - r;
                }
            }
        }
        load as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::from_str(input);
    platform.tilt_north();
    Some(platform.get_load())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::from_str(input);
    let mut cache = HashMap::new();
    for i in 0.. {
        if let Some(prev) = cache.get(&platform.map) {
            let cycle = i - prev;
            let remaining = (1000000000 - i) % cycle;
            for _ in 0..remaining {
                platform.tilt_cycle();
            }
            return Some(platform.get_load());
        }

        cache.insert(platform.map.clone(), i);
        platform.tilt_cycle();
    }
    Some(platform.get_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
