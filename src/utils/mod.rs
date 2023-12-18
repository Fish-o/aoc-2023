use itertools::Itertools;
use std::slice::Iter;
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
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "UP" | "U" | "N" | "NORTH" => Self::Up,
            "DOWN" | "D" | "S" | "SOUTH" => Self::Down,
            "LEFT" | "L" | "W" | "WEST" => Self::Left,
            "RIGHT" | "R" | "E" | "EAST" => Self::Right,
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
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}
impl<'a, 'b> std::ops::Add<&'a Direction> for &'b GridPoint {
    type Output = GridPoint;
    fn add(self, other: &Direction) -> GridPoint {
        match other {
            Direction::Up => GridPoint {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => GridPoint {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => GridPoint {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => GridPoint {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl std::ops::AddAssign<Direction> for GridPoint {
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
impl std::ops::Sub<&GridPoint> for &GridPoint {
    type Output = GridPoint;
    fn sub(self, other: &GridPoint) -> Self::Output {
        GridPoint {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GridPoint {
    x: usize,
    y: usize,
}
impl GridPoint {
    pub fn from_rc((row, col): (usize, usize)) -> Self {
        Self { x: col, y: row }
    }
    pub fn from_xy((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
    pub fn to_rc(&self) -> (usize, usize) {
        (self.y, self.x)
    }
    pub fn to_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn sub(&mut self, other: &GridPoint) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    x: i64,
    y: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn from_xy((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
    pub fn from_rc((row, col): (i64, i64)) -> Self {
        Self {
            x: col as i64,
            y: row as i64,
        }
    }
    pub fn to_xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }
    pub fn to_rc(&self) -> (i64, i64) {
        (self.y, self.x)
    }
    pub fn sub(&mut self, other: &Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
    pub fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl std::ops::Add<&Point> for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl std::ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, other: &Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl std::ops::Mul<i64> for &Point {
    type Output = Point;
    fn mul(self, other: i64) -> Self::Output {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl std::ops::MulAssign<i64> for Point {
    fn mul_assign(&mut self, other: i64) {
        self.x *= other;
        self.y *= other;
    }
}
impl std::ops::Add<&Direction> for &Point {
    type Output = Point;
    fn add(self, other: &Direction) -> Self::Output {
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
impl std::ops::Mul<usize> for &Direction {
    type Output = Point;
    fn mul(self, other: usize) -> Self::Output {
        match self {
            Direction::Up => Point {
                x: 0,
                y: -(other as i64),
            },
            Direction::Down => Point {
                x: 0,
                y: other as i64,
            },
            Direction::Left => Point {
                x: -(other as i64),
                y: 0,
            },
            Direction::Right => Point {
                x: other as i64,
                y: 0,
            },
        }
    }
}
impl std::ops::Mul<i64> for &Direction {
    type Output = Point;
    fn mul(self, other: i64) -> Self::Output {
        match self {
            Direction::Up => Point { x: 0, y: -other },
            Direction::Down => Point { x: 0, y: other },
            Direction::Left => Point { x: -other, y: 0 },
            Direction::Right => Point { x: other, y: 0 },
        }
    }
}
impl std::ops::MulAssign<usize> for Point {
    fn mul_assign(&mut self, other: usize) {
        self.x *= other as i64;
        self.y *= other as i64;
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
    pub fn get(&self, point: &GridPoint) -> Option<&char> {
        let (row, col) = point.to_rc();
        self.grid.get(row).and_then(|row| row.get(col))
    }

    pub fn get_mut(&mut self, point: &GridPoint) -> Option<&mut char> {
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
    pub fn is_valid_point(&self, point: &GridPoint) -> bool {
        let (row, col) = point.to_rc();
        if row >= self.grid.len() || col >= self.grid[0].len() {
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

pub fn shoelace(points: &[Point]) -> i64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    for i in 0..points.len() {
        let (x1, y1) = points[i].to_xy();
        let (x2, y2) = points[(i + 1) % points.len()].to_xy();
        area += (y1 + y2) * (x1 - x2);
    }
    area = (area / 2).abs();
    area
}
pub fn get_boundary(corners: &[Point]) -> i64 {
    corners
        .iter()
        .circular_tuple_windows()
        .fold(0, |acc, (p1, p2)| {
            let (x1, y1) = p1.to_xy();
            let (x2, y2) = p2.to_xy();
            acc + (x1 - x2).abs() + (y1 - y2).abs()
        })
}

pub fn get_inside_points(area: i64, boundary: i64) -> i64 {
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    area - (boundary / 2) + 1
}
