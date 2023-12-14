use itertools::Itertools;

advent_of_code::solution!(9);

pub fn get_value1(layer: &[i64]) -> i64 {
    if layer.iter().all(|el| el == &0) {
        return 0;
    }
    let new_layer = layer
        .iter()
        .tuple_windows()
        .map(|(el1, el2)| el2 - el1)
        .collect_vec();
    return get_value1(&new_layer) + *layer.last().unwrap();
}
pub fn get_value2(layer: &[i64]) -> i64 {
    if layer.iter().all(|el| el == &0) {
        return 0;
    }
    let new_layer = layer
        .iter()
        .tuple_windows()
        .map(|(el1, el2)| el2 - el1)
        .collect_vec();
    return layer.first().unwrap() - get_value2(&new_layer);
}

pub fn part_one(input: &str) -> Option<i64> {
    let sum = input
        .trim()
        .lines()
        .map(|line| {
            get_value1(
                &line
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sum = input
        .trim()
        .lines()
        .map(|line| {
            get_value2(
                &line
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
