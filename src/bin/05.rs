use std::{
    sync::{Arc, Mutex},
    u32::MAX,
};

use itertools::Itertools;

advent_of_code::solution!(5);
#[derive(Debug, Clone)]
struct AlmanacEntry {
    name: String,
    ranges: Vec<RangeMap>,
}

#[derive(Debug, Clone)]
struct RangeMap {
    start: u32,
    length: u32,
    destination: u32,
}

struct Range {
    start: u32,
    length: u32,
}
impl From<(u32, u32, u32)> for RangeMap {
    fn from((destination, start, length): (u32, u32, u32)) -> Self {
        Self {
            start,
            length,
            destination,
        }
    }
}

impl Range {
    pub fn apply(&self, al_range: RangeMap) -> Vec<Range> {
        todo!()
    }
}

impl AlmanacEntry {
    pub fn get_val(&self, val: u32) -> u32 {
        // destination range start of 50, a source range start of 98, and a range length
        for range in &self.ranges {
            if val >= range.start && val - range.start < range.length {
                return (val - range.start) + range.destination;
            }
        }
        return val;
    }
    pub fn map_ranges(&self, range: Range) -> Vec<(u32, u32)> {
        let mut containing_ranges = vec![];
        for al_range in &self.ranges {
            // Range:          x=======x
            // 1 Al               x-x
            // 2 Al               x--------x
            // 3 Al          x----x
            // 4 Al          x----------------x

            // 4
            if al_range.start < range.start
                && al_range.start + al_range.length > range.start + range.length
            {
                containing_ranges.push((al_range.start, al_range.length));
            }
        }
        //

        todo!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let seeds = sections.next().unwrap().trim();
    let seeds = seeds
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let sections = sections
        .map(|s| s.split("\n").filter(|&s| !s.is_empty()))
        .map(|mut s| {
            let s_name = s.next().unwrap();
            let s_ranges = s
                .map(|s| {
                    let mut numbers = s.split(" ").filter(|&s| !s.is_empty());
                    let low = numbers.next().unwrap();
                    let high = numbers.next().unwrap();
                    let res = numbers.next().unwrap();
                    (
                        low.parse::<u32>().unwrap(),
                        high.parse::<u32>().unwrap(),
                        res.parse::<u32>().unwrap(),
                    )
                        .into()
                })
                .collect::<Vec<_>>();
            AlmanacEntry {
                name: s_name.to_string(),
                ranges: s_ranges,
            }
        })
        .collect_vec();
    let mut lowest_location = MAX;
    for seed in seeds {
        let mut current_val = seed;
        for section in &sections {
            current_val = section.get_val(current_val);
        }
        if current_val < lowest_location {
            lowest_location = current_val;
        }
    }

    Some(lowest_location)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let seeds = sections.next().unwrap().trim();
    let mut seeds = seeds
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let sections = sections
        .map(|s| s.split("\n").filter(|&s| !s.is_empty()))
        .map(|mut s| {
            let s_name = s.next().unwrap();
            let s_ranges = s
                .map(|s| {
                    let mut numbers = s.split(" ").filter(|&s| !s.is_empty());
                    let low = numbers.next().unwrap();
                    let high = numbers.next().unwrap();
                    let res = numbers.next().unwrap();
                    (
                        low.parse::<u32>().unwrap(),
                        high.parse::<u32>().unwrap(),
                        res.parse::<u32>().unwrap(),
                    )
                        .into()
                })
                .collect::<Vec<_>>();
            AlmanacEntry {
                name: s_name.to_string(),
                ranges: s_ranges,
            }
        })
        .collect_vec();

    let mut seed_pairs = vec![];
    loop {
        if seeds.len() == 0 {
            break;
        }
        println!("Seeds left: {}", seeds.len());
        // Get first el of seeds
        let seed1 = seeds.remove(0);
        let seed2 = seeds.remove(0);
        seed_pairs.push((seed1, seed2));
    }
    // Divide seed pairs into threads
    let mut threads = vec![];
    let total_lowest = Arc::new(Mutex::new(MAX));
    for seed_pair in seed_pairs {
        let sections = sections.clone();
        let total_lowest = total_lowest.clone();
        threads.push(std::thread::spawn(move || {
            let mut lowest_found = MAX;
            let seed1 = seed_pair.0;
            let seed2 = seed_pair.1;
            for seed in seed1..(seed1 + seed2) {
                let mut current_val = seed;
                for section in &sections {
                    current_val = section.get_val(current_val);
                }
                if current_val < lowest_found {
                    lowest_found = current_val;
                }
            }
            let mut total_lowest = total_lowest.lock().unwrap();
            if lowest_found < *total_lowest {
                *total_lowest = lowest_found;
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let end_val = total_lowest.lock().unwrap().clone();
    Some(end_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
