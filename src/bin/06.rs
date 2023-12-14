advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let mut prod = 1;
    for (time, record) in times.zip(distances) {
        let mut min = 0;
        for time_spent_pressing in 0..time {
            if (time - time_spent_pressing) * (time_spent_pressing) > record {
                min = time_spent_pressing;
                break;
            }
        }
        let mut max = 0;
        for time_spent_pressing in 0..time {
            let time_spent_pressing = time - time_spent_pressing;
            if (time - time_spent_pressing) * (time_spent_pressing) > record {
                max = time_spent_pressing;
                break;
            }
        }
        prod *= (max - min) + 1;
    }
    Some(prod)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let mut prod = 1;
    let mut min = 0;
    for time_spent_pressing in 0..time {
        if (time - time_spent_pressing) * (time_spent_pressing) > record {
            min = time_spent_pressing;
            break;
        }
    }
    let mut max = 0;
    for time_spent_pressing in 0..time {
        let time_spent_pressing = time - time_spent_pressing;
        if (time - time_spent_pressing) * (time_spent_pressing) > record {
            max = time_spent_pressing;
            break;
        }
    }
    prod *= (max - min) + 1;
    Some(prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
