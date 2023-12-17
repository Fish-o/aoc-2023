advent_of_code::solution!(17);
use advent_of_code::utils::{Direction, Grid, Point};
use pathfinding::directed::astar::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: Point,
    arrived_from: Option<Direction>,
}

impl Node {
    fn successors(&self, map: &Grid) -> Vec<(Node, u32)> {
        // up to 3 units in every direction
        let mut nodes = Vec::new();
        for x in -3..=3 {
            let y = 0;
            if x == 0 {
                continue;
            }
            if let Some(dir) = &self.arrived_from {
                if dir == &Direction::Left || dir == &Direction::Right {
                    continue;
                }
            }

            let (s_x, s_y) = self.pos.to_xy();
            let (n_x, n_y) = (s_x as i32 + x, s_y as i32 + y);
            if n_x < 0 || n_y < 0 {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            let new_pos = Point::from_xy((n_x, n_y));
            if !map.is_valid_point(&new_pos) {
                continue;
            }

            let arrived_from = if x.is_negative() {
                Direction::Right
            } else {
                Direction::Left
            };
            let mut cost = 0;
            for i in 1..=x.abs() {
                let n_x = if x.is_negative() {
                    s_x - i as usize
                } else {
                    s_x + i as usize
                };
                let pos = Point::from_xy((n_x, s_y));
                if let Some(c) = map.get(&pos) {
                    cost += c.to_digit(10).unwrap();
                }
            }
            let node = Node {
                pos: new_pos,
                arrived_from: Some(arrived_from),
            };
            nodes.push((node, cost));
        }
        for y in -3..=3 {
            let x = 0;
            if y == 0 {
                continue;
            }
            if let Some(dir) = &self.arrived_from {
                if dir == &Direction::Down || dir == &Direction::Up {
                    continue;
                }
            }
            let (s_x, s_y) = self.pos.to_xy();
            let (n_x, n_y) = (s_x as i32 + x, s_y as i32 + y);
            if n_x < 0 || n_y < 0 {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            let new_pos = Point::from_xy((n_x, n_y));
            if !map.is_valid_point(&new_pos) {
                continue;
            }
            let mut cost = 0;
            for i in 1..=y.abs() {
                let n_y = if y.is_negative() {
                    s_y - i as usize
                } else {
                    s_y + i as usize
                };
                let pos = Point::from_xy((s_x, n_y));
                if let Some(c) = map.get(&pos) {
                    cost += c.to_digit(10).unwrap();
                }
            }
            let arrived_from = if y.is_negative() {
                Direction::Down
            } else {
                Direction::Up
            };
            let node = Node {
                pos: new_pos,
                arrived_from: Some(arrived_from),
            };
            nodes.push((node, cost));
        }
        nodes
    }

    fn ultra_successors(&self, map: &Grid) -> Vec<(Node, u32)> {
        // up to 3 units in every direction
        let mut nodes = Vec::new();
        for x in -10..=10 {
            let y = 0;
            if x == 0 || (x as i32).abs() < 4 as i32 {
                continue;
            }
            if let Some(dir) = &self.arrived_from {
                if dir == &Direction::Left || dir == &Direction::Right {
                    continue;
                }
            }
            let (s_x, s_y) = self.pos.to_xy();
            let (n_x, n_y) = (s_x as i32 + x, s_y as i32 + y);
            if n_x < 0 || n_y < 0 {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            let new_pos = Point::from_xy((n_x, n_y));
            if !map.is_valid_point(&new_pos) {
                continue;
            }

            let arrived_from = if x.is_negative() {
                Direction::Right
            } else {
                Direction::Left
            };
            let mut cost = 0;
            for i in 1..=x.abs() {
                let n_x = if x.is_negative() {
                    s_x - i as usize
                } else {
                    s_x + i as usize
                };
                let pos = Point::from_xy((n_x, s_y));
                if let Some(c) = map.get(&pos) {
                    cost += c.to_digit(10).unwrap();
                }
            }
            let node = Node {
                pos: new_pos,
                arrived_from: Some(arrived_from),
            };
            nodes.push((node, cost));
        }
        for y in -10..=10 {
            let x = 0;
            if y == 0 || (y as i32).abs() < 4 as i32 {
                continue;
            }
            if let Some(dir) = &self.arrived_from {
                if dir == &Direction::Down || dir == &Direction::Up {
                    continue;
                }
            }

            let (s_x, s_y) = self.pos.to_xy();
            let (n_x, n_y) = (s_x as i32 + x, s_y as i32 + y);
            if n_x < 0 || n_y < 0 {
                continue;
            }
            let (n_x, n_y) = (n_x as usize, n_y as usize);
            let new_pos = Point::from_xy((n_x, n_y));
            if !map.is_valid_point(&new_pos) {
                continue;
            }
            let mut cost = 0;
            for i in 1..=y.abs() {
                let n_y = if y.is_negative() {
                    s_y - i as usize
                } else {
                    s_y + i as usize
                };
                let pos = Point::from_xy((s_x, n_y));
                if let Some(c) = map.get(&pos) {
                    cost += c.to_digit(10).unwrap();
                }
            }
            let arrived_from = if y.is_negative() {
                Direction::Down
            } else {
                Direction::Up
            };
            let node = Node {
                pos: new_pos,
                arrived_from: Some(arrived_from),
            };
            nodes.push((node, cost));
        }
        nodes
    }
    fn distance_to_goal(&self, goal: &Point) -> u32 {
        let (x, y) = self.pos.to_xy();
        let (g_x, g_y) = goal.to_xy();
        ((x as i32 - g_x as i32).abs() + (y as i32 - g_y as i32).abs()) as u32
    }
    fn heuristic(&self, goal: &Point) -> u32 {
        let distance_to_goal = self.distance_to_goal(goal);
        return distance_to_goal;
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = Grid::new(input);
    let goal = Point::from_xy((map.get_grid()[0].len() - 1, map.get_grid().len() - 1));
    let start_node = Node {
        pos: Point::from_rc((0, 0)),
        arrived_from: None,
    };

    let result = astar(
        &start_node,
        |p| p.successors(&map),
        |p| p.heuristic(&goal),
        |p| p.pos == goal,
    );

    if let Some((_, cost)) = result {
        Some(cost)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Grid::new(input);
    let goal = Point::from_xy((map.get_grid()[0].len() - 1, map.get_grid().len() - 1));
    let start = Node {
        pos: Point::from_rc((0, 0)),
        arrived_from: None,
    };

    let result1 = astar(
        &start,
        |p| p.ultra_successors(&map),
        |p| p.heuristic(&goal),
        |p| p.pos == goal,
    );
    if let Some((_, cost)) = result1 {
        Some(cost)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
