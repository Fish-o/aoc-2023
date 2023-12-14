use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);
enum Dir {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.split_once("\n\n").unwrap();
    let lr = input
        .0
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut node_map = HashMap::new();
    for node in input.1.trim().lines() {
        let node = node.split_once("=").unwrap();
        let name = node.0.trim();
        let values = node.1.trim().split_once(", ").unwrap();
        let left = values.0.trim().replace("(", "");
        let right = values.1.trim().replace(")", "");
        node_map.insert(name, (left, right));
    }

    let mut position = ("AAA", node_map.get("AAA").unwrap());
    let mut i = 0;
    loop {
        let dir = lr.get(i % lr.len()).unwrap();
        i += 1;
        let new_pos = match dir {
            Dir::Left => position.1 .0.as_str(),
            Dir::Right => position.1 .1.as_str(),
        };
        position = (new_pos, node_map.get(new_pos).unwrap());
        if position.0 == "ZZZ" {
            break;
        }
    }
    Some(i as u32)
}

pub fn part_two(input: &str) -> Option<i128> {
    let input = input.split_once("\n\n").unwrap();
    let lr = input
        .0
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut node_map = HashMap::new();
    for node in input.1.trim().lines() {
        let node = node.split_once("=").unwrap();
        let name = node.0.trim();
        let values = node.1.trim().split_once(", ").unwrap();
        let left = values.0.trim().replace("(", "");
        let right = values.1.trim().replace(")", "");
        node_map.insert(name, (left, right));
    }
    let positions = node_map.keys().filter(|k| k.ends_with("A")).collect_vec();
    let i = positions
        .iter()
        .map(|p| {
            let mut i = 0;
            let mut pos = (**p, node_map.get(*p).unwrap());
            loop {
                let dir = lr.get(i % lr.len()).unwrap();
                i += 1;
                let new_pos = match dir {
                    Dir::Left => pos.1 .0.as_str(),
                    Dir::Right => pos.1 .1.as_str(),
                };
                if new_pos.ends_with("Z") {
                    break;
                }
                pos = (new_pos, node_map.get(new_pos).unwrap());
            }

            i as i128
        })
        .fold(1, num::integer::lcm);
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_pt("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
