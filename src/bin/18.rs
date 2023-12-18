use advent_of_code::utils::{get_boundary, get_inside_points, shoelace, Direction, Point};
use itertools::Itertools;

advent_of_code::solution!(18);

pub fn get_corners(instructions: Vec<(i64, Direction)>) -> Vec<Point> {
    instructions
        .iter()
        .scan(Point::new(0, 0), |point, (dis, dir)| {
            *point += &(dir * *dis);
            Some(point.clone())
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.trim().split_whitespace().collect_tuple().unwrap())
        .map(|(dir, dis, _)| (i64::from_str_radix(dis, 10).unwrap(), dir.into()))
        .collect_vec();
    let corners = get_corners(instructions);
    let area = shoelace(&corners);
    let boundary = get_boundary(&corners);
    Some(get_inside_points(area, boundary) + boundary)
}

pub fn part_two(input: &str) -> Option<i64> {
    static DIRS: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.trim().split_whitespace().collect_tuple().unwrap())
        .map(|(_, _, color)| color[2..color.len() - 1].to_string())
        .map(|color| (color[..5].to_string(), color.chars().nth(5).unwrap()))
        .map(|(dis, dir)| (i64::from_str_radix(&dis, 16), dir.to_digit(10).unwrap()))
        .map(|(dis, dir)| (dis.unwrap(), DIRS[dir as usize].clone()))
        .collect_vec();

    let corners = get_corners(instructions);
    let area = shoelace(&corners);

    let boundary = get_boundary(&corners);
    Some(get_inside_points(area, boundary) + boundary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
