use itertools::Itertools;

pub mod ranges;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rev(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn reverse(&mut self) {
        *self = self.rev();
    }

    pub fn from_nesw(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => Self::Up,
            'E' => Self::Right,
            'S' => Self::Down,
            'W' => Self::Left,
            _ => panic!("Invalid direction"),
        }
    }
    pub fn from_udlr(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
impl<'a, 'b> std::ops::Add<&'a Direction> for &'b Point {
    type Output = Point;
    fn add(self, other: &Direction) -> Point {
        match other {
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
impl std::ops::AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        match other {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

// Multiply dir by val
impl std::ops::Mul<usize> for Direction {
    type Output = (isize, isize);
    fn mul(self, other: usize) -> Self::Output {
        match self {
            Direction::Up => (0, -(other as isize)),
            Direction::Down => (0, other as isize),
            Direction::Left => (-(other as isize), 0),
            Direction::Right => (other as isize, 0),
        }
    }
}
impl std::ops::Mul<isize> for Direction {
    type Output = (isize, isize);
    fn mul(self, other: isize) -> Self::Output {
        match self {
            Direction::Up => (0, -other),
            Direction::Down => (0, other),
            Direction::Left => (-other, 0),
            Direction::Right => (other, 0),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    x: isize,
    y: isize,
}
impl Point {
    pub fn from_rc((row, col): (usize, usize)) -> Self {
        Self {
            x: col as isize,
            y: row as isize,
        }
    }
    pub fn from_xy((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn to_rc(&self) -> (usize, usize) {
        let x = if self.x.is_negative() {
            0
        } else {
            self.x as usize
        };
        let y = if self.y.is_negative() {
            0
        } else {
            self.y as usize
        };

        (y, x)
    }
    pub fn to_xy(&self) -> (usize, usize) {
        let (row, col) = self.to_rc();
        (col, row)
    }
    pub fn to_xy_isize(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub fn to_rc_isize(&self) -> (isize, isize) {
        (self.y, self.x)
    }

    pub fn has_negative(&self) -> bool {
        self.x.is_negative() || self.y.is_negative()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self { grid }
    }
    pub fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }
    pub fn get(&self, point: &Point) -> Option<&char> {
        let (row, col) = point.to_rc();
        self.grid.get(row).and_then(|row| row.get(col))
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut char> {
        let (row, col) = point.to_rc();
        self.grid.get_mut(row).and_then(|row| row.get_mut(col))
    }
    pub fn rotate(&mut self) {
        let mut new_grid = vec![vec![' '; self.grid.len()]; self.grid[0].len()];
        for (row, line) in self.grid.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                new_grid[col][self.grid.len() - row - 1] = *c;
            }
        }
        self.grid = new_grid;
    }
    pub fn get_grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
    pub fn get_grid_mut(&mut self) -> &mut Vec<Vec<char>> {
        &mut self.grid
    }
    pub fn get_row(&self, row: usize) -> Option<&Vec<char>> {
        self.grid.get(row)
    }
    pub fn get_col(&self, col: usize) -> Option<Vec<char>> {
        self.grid
            .iter()
            .map(|row| *row.get(col).expect("Invalid column"))
            .collect_vec()
            .into()
    }
    pub fn is_valid_point(&self, point: &Point) -> bool {
        let (row, col) = point.to_rc();
        if point.has_negative() || row >= self.grid.len() || col >= self.grid[0].len() {
            return false;
        }
        true
    }
    pub fn print(&self) {
        for line in &self.grid {
            println!("{}", line.iter().join(""));
        }
    }
}
