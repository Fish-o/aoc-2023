advent_of_code::solution!(16);
use std::collections::HashMap;

use advent_of_code::utils::{Direction, Grid, GridPoint};

fn solve(
    board: &Grid,
    dir: &Direction,
    pos: &GridPoint,
    visited: &mut HashMap<GridPoint, Vec<Direction>>,
) {
    let mut current_char = board.get(pos).unwrap();
    let specials = ['/', '\\', '|', '-'];
    let mut pos = pos.clone();
    while !specials.contains(&current_char) {
        visited.insert(pos.clone(), vec![]);
        pos = &pos + dir;
        if board.is_valid_point(&pos) {
            current_char = board.get(&pos).unwrap();
        } else {
            return;
        }
    }
    let vec = visited.entry(pos.clone()).or_insert(vec![]);
    if current_char == &'/' {
        let new_dir = match dir {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Left,
        };
        if vec.contains(&new_dir) {
            return;
        }
        vec.push(new_dir.clone());
        let new_pos = &pos + &new_dir;
        if board.is_valid_point(&new_pos) {
            solve(board, &new_dir, &new_pos, visited);
        }
    } else if current_char == &'\\' {
        let new_dir = match dir {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Left,
        };
        if vec.contains(&new_dir) {
            return;
        }
        vec.push(new_dir.clone());
        let new_pos = &pos + &new_dir;
        if board.is_valid_point(&new_pos) {
            solve(board, &new_dir, &new_pos, visited);
        }
    } else if current_char == &'|' {
        let new_up_pos = &pos + &Direction::Up;
        let new_down_pos = &pos + &Direction::Down;
        if board.is_valid_point(&new_up_pos) && !matches!(dir, &Direction::Down) {
            solve(board, &Direction::Up, &new_up_pos, visited);
        }
        if board.is_valid_point(&new_down_pos) && !matches!(dir, &Direction::Up) {
            solve(board, &Direction::Down, &new_down_pos, visited);
        }
    } else if current_char == &'-' {
        let new_left_pos = &pos + &Direction::Left;
        let new_right_pos = &pos + &Direction::Right;
        if board.is_valid_point(&new_left_pos) && !matches!(dir, &Direction::Right) {
            solve(board, &Direction::Left, &new_left_pos, visited);
        }
        if board.is_valid_point(&new_right_pos) && !matches!(dir, &Direction::Left) {
            solve(board, &Direction::Right, &new_right_pos, visited);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Grid::new(input);
    let mut visited = HashMap::new();
    solve(
        &board,
        &Direction::Right,
        &GridPoint::from_rc((0, 0)),
        &mut visited,
    );
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Grid::new(input);
    let (row_len, col_len) = board.size();
    let mut max = 0;
    // Down
    for j in 0..col_len {
        let mut visited = HashMap::new();
        solve(
            &board,
            &Direction::Down,
            &GridPoint::from_rc((0, j)),
            &mut visited,
        );
        max = max.max(visited.len());
    }
    // Up
    for j in 0..col_len {
        let mut visited = HashMap::new();
        solve(
            &board,
            &Direction::Up,
            &GridPoint::from_rc((row_len - 1, j)),
            &mut visited,
        );
        max = max.max(visited.len());
    }
    // Left
    for i in 0..row_len {
        let mut visited = HashMap::new();
        solve(
            &board,
            &Direction::Left,
            &GridPoint::from_rc((i, col_len - 1)),
            &mut visited,
        );
        max = max.max(visited.len());
    }
    // Right
    for i in 0..row_len {
        let mut visited = HashMap::new();
        solve(
            &board,
            &Direction::Right,
            &GridPoint::from_rc((i, 0)),
            &mut visited,
        );
        max = max.max(visited.len());
    }
    Some(max as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
