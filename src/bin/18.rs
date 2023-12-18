use advent_of_code::utils::{Direction, Point};
use itertools::Itertools;

advent_of_code::solution!(18);
fn walk_path(input: &str) -> Vec<Point> {
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.trim().split(' ').collect_tuple().unwrap())
        .map(|(dir, dist, col)| {
            let dist = dist.parse::<u32>().unwrap();
            // Remove first and last char of col
            let col = col[1..col.len() - 1].to_string();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            };
            (dir, dist, col)
        })
        .collect_vec();

    let mut pos = Point::from_rc((1000, 1000));
    let mut path = vec![pos.clone()];
    for instruction in instructions {
        let (dir, dist, _) = instruction;
        for _ in 0..dist {
            pos = &pos + &dir;
            path.push(pos.clone());
        }
    }
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    for p in &path {
        let (x, y) = p.to_xy();
        if x < min_x {
            min_x = x;
        }

        if y < min_y {
            min_y = y;
        }
    }
    let min = Point::from_xy((min_x, min_y));
    for p in &mut path {
        p.sub(&min);
    }
    path.sort();
    path.dedup();
    path
}
pub fn get_corners(input: &str) -> Vec<(i64, i64)> {
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.trim().split(' ').collect_tuple().unwrap())
        .map(|(dir, dist, col)| {
            // let dist = dist.parse::<u32>().unwrap();
            let col = col[2..col.len() - 1].to_string();
            // let dir = Direction::from_udlr(dir.chars().next().unwrap());
            col
        })
        .map(|col| {
            let first_five = col.chars().take(5).collect::<String>();
            let last = col.chars().last().unwrap();
            // Parse first_five as hex
            let first_five = u32::from_str_radix(&first_five, 16).unwrap();
            let last = u32::from_str_radix(&last.to_string(), 16).unwrap();
            let last = match last {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            (last, first_five)
        })
        .collect_vec();

    let mut pos: (i64, i64) = (0, 0);
    let mut points = vec![(0, 0)];
    for instruction in instructions {
        let (dir, dist) = instruction;
        let (x, y) = pos;
        let (x, y) = match dir {
            Direction::Up => (x, y - dist as i64),
            Direction::Down => (x, y + dist as i64),
            Direction::Left => (x - dist as i64, y),
            Direction::Right => (x + dist as i64, y),
        };
        pos = (x, y);
        points.push((x, y));
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    let path = walk_path(input);
    let mut grid = vec![vec![' '; 1000]; 1000];
    for p in &path {
        let (x, y) = p.to_xy();
        grid[y as usize][x as usize] = '#';
    }
    // Print grid
    // for line in &grid {
    //     for c in line {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    // Get point inside
    let mut point_inside = None;
    for (r, row) in grid.iter().enumerate() {
        let mut inside = false;
        for (c, tile) in row.iter().enumerate() {
            if *tile == '#' {
                if !inside {
                    inside = true;
                } else {
                    break;
                }
            } else if *tile == ' ' && inside {
                point_inside = Some(Point::from_rc((r, c)));
                break;
            }
        }
    }
    let point_inside = point_inside.unwrap();
    // Flood fill
    let mut queue = vec![point_inside];
    let mut visited = vec![vec![false; 1000]; 1000];
    let mut count = 0;
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        let (x, y) = p.to_xy();
        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;
        count += 1;
        for dir in Direction::iter() {
            let new_p = &p + dir;
            let (x, y) = new_p.to_xy();
            if grid[y as usize][x as usize] == ' ' {
                queue.push(new_p);
            }
        }
    }

    // Print grid
    // for (r, line) in grid.iter().enumerate() {
    //     for (c, tile) in line.iter().enumerate() {
    //         if visited[r][c] {
    //             print!("O");
    //         } else {
    //             print!("{}", tile);
    //         }
    //     }
    //     println!();
    // }
    Some(count + path.len() as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let points = get_corners(input);
    // Shoelace formula to get the area
    let mut area = 0;
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];
        area += x1 * y2;
        area -= x2 * y1;
    }
    area = (area / 2).abs();

    // Get boundary points
    let mut boundary_points = 0;
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];
        boundary_points += (x1 - x2).abs().max((y1 - y2).abs());
    }
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    let i = area - boundary_points / 2 + 1;
    Some(i + boundary_points)
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
