advent_of_code::solution!(10);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        }
    }
}
enum Tile {
    Empty,
    Pipe(Direction, Direction),
    Start,
}
struct Map {
    map: Vec<Vec<Tile>>,
}
impl Map {
    pub fn from_str(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '|' => Tile::Pipe(Direction::Up, Direction::Down),
                        '-' => Tile::Pipe(Direction::Left, Direction::Right),
                        'F' => Tile::Pipe(Direction::Down, Direction::Right),
                        '7' => Tile::Pipe(Direction::Down, Direction::Left),
                        'J' => Tile::Pipe(Direction::Up, Direction::Left),
                        'L' => Tile::Pipe(Direction::Up, Direction::Right),
                        'S' => Tile::Start,
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();
        Self { map }
    }
    pub fn find_start(&self) -> (usize, usize) {
        for (x, row) in self.map.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if let Tile::Start = tile {
                    return (x, y);
                }
            }
        }
        unreachable!()
    }
    pub fn get_tiles_around(&self, (x, y): (usize, usize)) -> Vec<((usize, usize), Direction)> {
        let mut tiles = Vec::new();
        if x > 0 {
            tiles.push(((x - 1, y), Direction::Up));
        }
        if y > 0 {
            tiles.push(((x, y - 1), Direction::Left));
        }
        if x < self.map[0].len() - 1 {
            tiles.push(((x + 1, y), Direction::Down));
        }
        if y < self.map.len() - 1 {
            tiles.push(((x, y + 1), Direction::Right));
        }
        tiles
    }
    pub fn get_tile(&self, (x, y): (usize, usize)) -> &Tile {
        &self.map[x][y]
    }
    pub fn reverse_dir(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn find_furthest_node(&self) -> (u32, Vec<(usize, usize)>, (Direction, Direction)) {
        let mut tiles = Vec::new();
        let mut front = (0, 0);
        let mut front_from_dir = Direction::Up;

        let mut back = (0, 0);
        let mut back_from_dir = Direction::Up;
        let start = self.find_start();
        for (pos, dir) in self.get_tiles_around(start) {
            if let Tile::Pipe(a, b) = self.get_tile(pos) {
                let mut touching = false;
                if *a == Self::reverse_dir(dir) || *b == Self::reverse_dir(dir) {
                    touching = true;
                }

                if touching {
                    if front == (0, 0) {
                        front = pos;
                        front_from_dir = dir;
                    } else {
                        back = pos;
                        back_from_dir = dir;
                        break;
                    }
                }
            }
        }
        let s_type = (back_from_dir, front_from_dir);
        tiles.push(front);
        tiles.push(back);
        tiles.push(start);
        for step in 2.. {
            (front, front_from_dir) = match self.get_tile(front) {
                Tile::Pipe(a, b) => {
                    if Self::reverse_dir(*a) == front_from_dir {
                        (b.apply(front), *b)
                    } else {
                        (a.apply(front), *a)
                    }
                }
                _ => panic!("Invalid tile"),
            };
            (back, back_from_dir) = match self.get_tile(back) {
                Tile::Pipe(a, b) => {
                    if Self::reverse_dir(*a) == back_from_dir {
                        (b.apply(back), *b)
                    } else {
                        (a.apply(back), *a)
                    }
                }
                _ => panic!("Invalid tile"),
            };
            if front == back {
                tiles.push(front);
                return (step, tiles, s_type);
            } else {
                tiles.push(front);
                tiles.push(back);
            }
        }
        unreachable!()
    }

    pub fn find_inside_area(&self, input_str: &str) -> u32 {
        // O |  I
        // O || O
        // O L7 I
        // I FJ O
        // O F7 O

        let (_, mut tiles, s_type) = self.find_furthest_node();
        tiles.sort_unstable();
        let s = match s_type {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => "|",
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => "-",
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => "F",
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => "7",
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => "J",
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => "L",
            _ => panic!("Invalid type"),
        };

        input_str
            .lines()
            .enumerate()
            .map(|(x, row)| {
                let mut inside = false;
                let mut count = 0;
                let mut action = None;
                let row = row.replace('S', s);
                for (y, c) in row.chars().enumerate() {
                    if !tiles.binary_search(&(x, y)).is_ok() {
                        if inside {
                            count += 1;
                        }
                    } else {
                        match (c, action) {
                            ('.', _) => {
                                if inside {
                                    count += 1;
                                }
                            }
                            ('|', _) => {
                                inside = !inside;
                            }
                            ('-', _) => {}
                            ('F', _) => {
                                action = Some('F');
                            }
                            ('L', _) => {
                                action = Some('L');
                            }
                            ('7', Some('F')) => {}
                            ('7', Some('L')) => {
                                inside = !inside;
                            }
                            ('J', Some('F')) => {
                                inside = !inside;
                            }
                            ('J', Some('L')) => {}
                            (a, b) => {
                                unreachable!("Invalid tile: {:?} {:?}", a, b);
                            }
                        }
                    }
                }
                count
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from_str(input);
    Some(map.find_furthest_node().0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_str(input);
    let sum = map.find_inside_area(input);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_pt("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
