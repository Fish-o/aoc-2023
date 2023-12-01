use std::string;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|f| f.chars().collect::<Vec<_>>())
            .map(|c| c.into_iter().filter(|f| f.clone().is_numeric()))
            .map(|c| c.collect::<Vec<char>>())
            .filter(|c| c.len() >= 1)
            .map(|c| format!("{}{}", c.first().unwrap(), c.last().unwrap()))
            .map(|s| s.parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|s| {
                if s.chars().filter(|c| c.is_alphabetic()).count() < 3 {
                    return s.to_string();
                }
                let mut changed = s
                    .chars()
                    .tuple_windows()
                    .map(|(c1, c2, c3, c4, c5)| match (c1, c2, c3, c4, c5) {
                        ('o', 'n', 'e', _, _) => '1',
                        ('t', 'w', 'o', _, _) => '2',
                        ('t', 'h', 'r', 'e', 'e') => '3',
                        ('f', 'o', 'u', 'r', _) => '4',
                        ('f', 'i', 'v', 'e', _) => '5',
                        ('s', 'i', 'x', _, _) => '6',
                        ('s', 'e', 'v', 'e', 'n') => '7',
                        ('e', 'i', 'g', 'h', 't') => '8',
                        ('n', 'i', 'n', 'e', _) => '9',
                        _ => c1,
                    })
                    .collect::<Vec<_>>();
                let mut unmapped = s.chars().skip(changed.len());
                let first = unmapped.next().expect("First");
                let second = unmapped.next().expect("Second");
                let third = unmapped.next().expect("Third");
                let fourth: Option<char> = unmapped.next();
                changed.push(first);
                changed.push(match (first, second, third, fourth) {
                    ('o', 'n', 'e', _) => '1',
                    ('t', 'w', 'o', _) => '2',
                    ('f', 'o', 'u', Some('r')) => '4',
                    ('f', 'i', 'v', Some('e')) => '5',
                    ('s', 'i', 'x', _) => '6',
                    ('n', 'i', 'n', Some('e')) => '9',
                    (_, 'o', 'n', Some('e')) => '1',
                    (_, 't', 'w', Some('o')) => '2',
                    (_, 's', 'i', Some('x')) => '6',
                    _ => second,
                });
                changed.push(third);
                if fourth.is_some() {
                    changed.push(fourth.unwrap());
                }
                changed.iter().collect::<String>()
            })
            .map(|f| f.chars().collect::<Vec<_>>())
            .map(|c| c.into_iter().filter(|f| f.clone().is_numeric()))
            .map(|c| c.collect::<Vec<char>>())
            .filter(|c| c.len() >= 1)
            .map(|c| format!("{}{}", c.first().unwrap(), c.last().unwrap()))
            .map(|s| s.parse::<u32>().expect("Parsable"))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
