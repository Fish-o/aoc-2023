use itertools::Itertools;

advent_of_code::solution!(5);
#[derive(Debug, Clone)]
struct AlmanacEntry {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug, Clone)]
struct RangeMap {
    pub start: i64,
    pub length: i64,
    pub destination: i64,
}

impl From<(i64, i64, i64)> for RangeMap {
    fn from((destination, start, length): (i64, i64, i64)) -> Self {
        Self {
            start,
            length,
            destination,
        }
    }
}

// Source: https://github.com/orlp/aoc2023/blob/master/src/bin/day05.rs
fn min_location_rec(mut range: (i64, i64), maps: &[Vec<(i64, i64, i64)>]) -> i64 {
    if range.0 >= range.1 {
        return i64::MAX;
    }
    let Some(map) = maps.first() else {
        return range.0;
    };

    let mut bound = i64::MAX;
    for (dst, src, len) in map.iter().copied() {
        let (start, stop) = range;
        if start >= stop {
            return bound;
        }
        let before = (start.min(src), stop.min(src));
        let overlap = (start.max(src), stop.min(src + len));
        let after = (start.max(src + len), stop.max(src + len));

        bound = bound.min(min_location_rec(before, &maps[1..]));
        let map_overlap = (overlap.0 - src + dst, overlap.1 - src + dst);
        bound = bound.min(min_location_rec(map_overlap, &maps[1..]));
        range = after;
    }

    bound.min(min_location_rec(range, &maps[1..]))
}

impl AlmanacEntry {
    pub fn get_val(&self, val: i64) -> i64 {
        // destination range start of 50, a source range start of 98, and a range length
        for range in &self.ranges {
            if val >= range.start && val - range.start < range.length {
                return (val - range.start) + range.destination;
            }
        }
        return val;
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut sections = input.split("\n\n");
    let seeds = sections.next().unwrap().trim();
    let seeds = seeds
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    let sections = sections
        .map(|s| s.split("\n").filter(|&s| !s.is_empty()))
        .map(|mut s| {
            let _ = s.next().unwrap();
            let mut s_ranges = s
                .map(|s| {
                    let mut numbers = s.split(" ").filter(|&s| !s.is_empty());
                    let dst = numbers.next().unwrap();
                    let src = numbers.next().unwrap();
                    let len = numbers.next().unwrap();
                    (
                        dst.parse::<i64>().unwrap(),
                        src.parse::<i64>().unwrap(),
                        len.parse::<i64>().unwrap(),
                    )
                        .into()
                })
                .collect::<Vec<RangeMap>>();
            s_ranges.sort_by_key(|f| f.start);

            AlmanacEntry { ranges: s_ranges }
        })
        .collect_vec();

    let mut lowest_location = i64::MAX;
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

pub fn part_two(input: &str) -> Option<i64> {
    let mut sections = input.split("\n\n");
    let seeds = sections.next().unwrap().trim();
    let seeds = seeds
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    for map in maps.iter_mut() {
        map.sort_unstable_by_key(|(_dst, src, _len)| *src);
    }
    maps = sections
        .map(|s| s.split("\n").filter(|&s| !s.is_empty()))
        .map(|mut s| {
            let _ = s.next().unwrap();
            let s_ranges = s
                .filter(|s| !s.is_empty())
                .map(|s| s.split_whitespace())
                .map(|mut s| {
                    (
                        s.next().unwrap().parse::<i64>().unwrap(),
                        s.next().unwrap().parse::<i64>().unwrap(),
                        s.next().unwrap().parse::<i64>().unwrap(),
                    )
                        .into()
                })
                .collect::<Vec<_>>();
            s_ranges
        })
        .collect_vec();
    for map in &mut maps {
        map.sort_by_key(|f| f.1)
    }

    let min_loc = |r| min_location_rec(r, &maps);
    let res = seeds.chunks_exact(2).map(|c| min_loc((c[0], c[0] + c[1])));
    res.min()
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
