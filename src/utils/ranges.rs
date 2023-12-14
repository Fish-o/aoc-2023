use std::iter::FromIterator;
use std::{cmp, fmt};

#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Range {
        Range {
            start,
            end,
        }
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }

    pub fn merge(&mut self, other: &Range) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.start, self.end)
    }
}

#[derive(Debug, Clone)]
pub struct RangeStack {
    pub ranges: Vec<Range>,
}

impl Default for RangeStack {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeStack {
    pub fn add(&mut self, range: &Range) {
        if let Some(last) = self.ranges.last_mut() {
            if last.overlaps(range) {
                last.merge(range);
                return;
            }
        }

        self.ranges.push(*range);
    }
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }
}

impl fmt::Display for RangeStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for range in &self.ranges {
            write!(f, " {}", range)?;
        }
        Ok(())
    }
}

impl FromIterator<Range> for RangeStack {
    fn from_iter<I>(iterator: I) -> Self
    where
        I: IntoIterator<Item = Range>,
    {
        let mut raw_ranges: Vec<_> = iterator.into_iter().collect();
        raw_ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let mut range_stack = RangeStack { ranges: Vec::new() };

        for range in &raw_ranges {
            range_stack.add(range);
        }

        range_stack
    }
}

impl<'a> FromIterator<&'a Range> for RangeStack {
    fn from_iter<I>(iterator: I) -> Self
    where
        I: IntoIterator<Item = &'a Range>,
    {
        iterator.into_iter().cloned().collect()
    }
}
