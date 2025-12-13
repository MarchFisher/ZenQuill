use std::error::Error;
use std::fs::read_to_string;

use crate::core::line::Line;

/// buffer
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(file_name)?;

        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Line::from(line));
        }

        Ok(Self { lines })
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, character: char, location: super::view::Location) {
        if location.line_index > self.lines.len() {
            return ;
        }
        if location.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(location.line_index) {
            line.insert_char(character, location.grapheme_index);
        }
    }

    pub fn delete_char(&mut self, location: super::view::Location) {
        if let Some(line) = self.lines.get(location.line_index) {
            if location.grapheme_index >= line.grapheme_count() && 
               self.lines.len() > location.line_index.saturating_add(1) {

                let next_line = self.lines.remove(location.line_index.saturating_add(1));
                self.lines[location.line_index].append(&next_line);

            } else if location.grapheme_index < line.grapheme_count() {

                self.lines[location.line_index].delete(location.grapheme_index);
                
            }
        }
    }
}