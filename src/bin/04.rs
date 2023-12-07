use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(4);

fn parse_data(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.split(':'))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .map(|(_, card)| {
            let mut card = card.split('|');
            let winning = card
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let having = card
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (winning, having)
        })
        .map(|(winning, having)| winning.iter().filter(|&n| having.contains(n)).count())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_data(input)
            .iter()
            .map(|x| 1 << x)
            .map(|x| x >> 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut card_count: HashMap<usize, u32> = HashMap::new();
    for (index, overlapping) in parse_data(input).iter().enumerate() {
        let amount = card_count.get(&index).unwrap_or(&0) + 1;
        sum += amount;
        for i in (index + 1)..=(index + overlapping) {
            let current_amount = card_count.get(&i).unwrap_or(&0);
            card_count.insert(i, amount + current_amount);
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
