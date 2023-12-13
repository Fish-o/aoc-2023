advent_of_code::solution!(13);
struct Pattern {
    map: Vec<Vec<char>>,
}
impl Pattern {
    pub fn from_str(input: &str) -> Self {
        let mut pattern = Self { map: vec![] };
        for row in input.lines() {
            let mut m_row = vec![];
            for char in row.chars() {
                m_row.push(char);
            }
            pattern.map.push(m_row);
        }
        pattern
    }

    pub fn find_row_reflects(&self) -> Vec<usize> {
        let mut reflects = vec![];
        let first_row = self.map[0].clone();
        for (index, row) in self.map.iter().enumerate().skip(1) {
            if row == &first_row {
                let mut correct = true;
                let first_matching_rows = (index + 1) / 2;
                for i in 0..first_matching_rows {
                    if self.map.get(i).unwrap()
                        != self
                            .map
                            .get(first_matching_rows + first_matching_rows - 1 - i)
                            .unwrap()
                    {
                        correct = false;
                        break;
                    }
                }
                if !correct {
                    continue;
                }
                reflects.push(first_matching_rows);
            }
        }
        let last_row = self.map.last().unwrap().clone();
        for (index, row) in self.map.iter().enumerate().take(self.map.len() - 1).skip(1) {
            if row == &last_row {
                let last_matching_rows = (self.map.len() + index) / 2;
                if last_matching_rows < self.map.len() / 2 {
                    continue;
                }
                let mut correct = true;

                for i in last_matching_rows..self.map.len() {
                    let mirror_below_i = last_matching_rows - (i - last_matching_rows) - 1;
                    if self.map.get(i).unwrap() != self.map.get(mirror_below_i).unwrap() {
                        correct = false;
                        break;
                    }
                }
                if !correct {
                    continue;
                }
                reflects.push(last_matching_rows);
            }
        }
        return reflects;
    }
    pub fn find_col_reflect(&mut self) -> Vec<usize> {
        self.rotate();
        self.find_row_reflects()
    }
    fn rotate(&mut self) {
        let mut new_map = vec![];
        for i in 0..self.map[0].len() {
            let mut new_row = vec![];
            for j in 0..self.map.len() {
                new_row.push(self.map[j][i]);
            }
            new_map.push(new_row);
        }
        self.map = new_map;
    }

    pub fn brute_force_smudge(&mut self) -> usize {
        let row = self.find_row_reflects();
        let map = self.map.clone();
        let col = self.find_col_reflect();
        self.map = map;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                let map = self.map.clone();
                let val = self.map[i][j];
                if val == '#' {
                    self.map[i][j] = '.';
                } else {
                    self.map[i][j] = '#';
                }
                let new_row_res = self.find_row_reflects();
                let new_col_res = self.find_col_reflect();
                for new_row in new_row_res {
                    if !row.contains(&new_row) {
                        return new_row * 100;
                    }
                }
                for new_col in new_col_res {
                    if !col.contains(&new_col) {
                        return new_col;
                    }
                }
                self.map = map;
                // Col with prev row
            }
        }
        unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .split("\n\n")
        .map(|s| Pattern::from_str(s))
        .map(|mut p| {
            (p.find_row_reflects().first().unwrap_or(&0) * 100)
                + p.find_col_reflect().first().unwrap_or(&0)
        })
        .sum::<usize>();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .split("\n\n")
        .map(|s| Pattern::from_str(s))
        .map(|mut p| p.brute_force_smudge())
        .sum::<usize>();
    Some(res as u32)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
