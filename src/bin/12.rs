use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn map_verifies_condition(map: &str, condition: &[u32]) -> bool {
    let broken = map.split('.').filter(|l| !l.is_empty()).collect_vec();
    if broken.len() != condition.len() {
        return false;
    } else {
        for (i, b) in broken.iter().enumerate() {
            if b.len() != condition[i] as usize {
                return false;
            }
        }
    }
    true
}

pub fn get_permutations(
    map: &String,
    condition: &[u32],
    cache: &mut HashMap<(String, Vec<u32>), i64>,
) -> i64 {
    let key = (map.to_owned(), condition.to_vec());
    if let Some(c) = cache.get(&key) {
        return *c;
    }
    let mut broken_count = 0;
    let mut condition_i = 0;
    for (index, c) in map.chars().enumerate() {
        match c {
            '.' => {
                if broken_count > 0 {
                    if condition_i >= condition.len() || broken_count != condition[condition_i] {
                        return 0;
                    }
                    broken_count = 0;
                    condition_i += 1;
                }
            }
            '#' => broken_count += 1,
            '?' => {
                let new_map1 = "#".repeat(broken_count as usize) + "#" + &map[(index + 1)..];
                let opt1 = get_permutations(&new_map1, &condition[condition_i..], cache);
                let opt1_key = (new_map1.clone(), condition[condition_i..].to_vec());
                cache.insert(opt1_key, opt1);
                let new_map2 = map[(index + 1)..].to_owned();
                if broken_count > 0 {
                    if condition_i >= condition.len() || broken_count != condition[condition_i] {
                        return opt1;
                    }
                    condition_i += 1;
                }
                let opt2 = get_permutations(&new_map2, &condition[condition_i..], cache);
                let opt2_key = (new_map2, condition[condition_i..].to_vec());
                cache.insert(opt2_key, opt2);
                return opt2 + opt1;
            }
            _ => unreachable!(),
        }
    }
    if condition.len() - condition_i == 0 && broken_count == 0
        || condition.len() - condition_i == 1 && broken_count == condition[condition_i]
    {
        1
    } else {
        0
    }
}

fn expand_map(map: &str) -> String {
    let new_map = map.to_string() + "?";
    new_map.repeat(5)[..(new_map.len() * 5 - 1)].to_string()
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|l| {
            let l = l.split_once(' ').unwrap();
            let map = l.0.trim();
            let group_sizes =
                l.1.split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec();
            get_permutations(&map.to_owned(), &group_sizes, &mut cache)
        })
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut cache = HashMap::new();
    Some(
        input
            .lines()
            .map(|line| {
                let l = line.split_once(' ').unwrap();
                let map = expand_map(l.0.trim());
                let group_sizes =
                    l.1.split(',')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect_vec()
                        .repeat(5);
                get_permutations(&map.to_owned(), &group_sizes, &mut cache)
            })
            .sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
