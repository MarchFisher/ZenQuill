use std::ops::Range;

#[derive(Default)]
pub struct Line {
    string: String,
}

impl Line {

    pub fn from(line_str: &str) -> Self {
        Self { string: line_str.to_string() }
    }

/// **Not very safe**
    pub fn get(&self, range: Range<usize>) -> &str {
        let start = std::cmp::min(range.start, self.string.len());
        let end = std::cmp::min(range.end, self.string.len());

        self.string.get(start..end).unwrap_or_default()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}