use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split(',')
            .map(|s| {
                let mut val: u32 = 0;
                let iter = s.chars().map(|c| c as u8);
                for count in iter {
                    val += count as u32;
                    val = val * 17;
                    val = val % 256;
                }
                val
            })
            .collect_vec()
            .iter()
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let res = input.trim().split(',').map(|s| {
        let mut box_i: u32 = 0;

        let mut label = String::new();
        let mut operation = ' ';
        for c in s.chars() {
            match c {
                '-' | '=' => {
                    operation = c;
                    break;
                }
                _ => {}
            }
            label.push(c);
        }

        let iter = label.chars().map(|c| c as u8);
        for count in iter {
            box_i += count as u32;
            box_i = box_i * 17;
            box_i = box_i % 256;
        }
        if operation == '=' {
            //
            let size = s.split_once('=').unwrap().1.parse::<u32>().unwrap();
            (box_i, operation, label, size)
        } else {
            (box_i, operation, label, 0)
        }
    });
    let mut lookup: HashMap<u32, Vec<(String, u32)>> = HashMap::new();
    for item in res {
        let (box_i, operation, label, size) = item;
        let entry = lookup.entry(box_i).or_insert(Vec::new());
        match operation {
            '-' => {
                let matching = entry
                    .iter()
                    .enumerate()
                    .find(|(_, (l, _))| l == &label)
                    .map(|(i, _)| i);
                if let Some(i) = matching {
                    entry.remove(i);
                }
            }
            '=' => {
                let matching = entry.iter().enumerate().find(|(_, (l, _))| l == &label);
                if let Some((i, (l, _))) = matching {
                    entry[i] = (l.clone(), size);
                } else {
                    entry.push((label, size));
                }
            }
            _ => {}
        }
    }
    let mut sum: i64 = 0;
    for (box_i, lenses) in lookup {
        for (i, (_, f)) in lenses.iter().enumerate() {
            let mut val = (box_i as i64) + 1;
            val = val * ((i + 1) as i64);
            val = val * (*f as i64);
            sum += val;
        }
    }
    Some(sum)
}

// 4249243 INCORRECT

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
