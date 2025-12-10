use std::error::Error;

use crate::core::Position;
use crate::core::command::{Direction, EditorCommand};

use crate::core::terminal::{ Size, Terminal };
use crate::core::buffer::Buffer;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

impl From<Position> for Location {
    fn from(value: Position) -> Self {
        Self { grapheme_index: value.col, line_index: value.row }
    }
}

pub struct View{
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        View { 
            buffer: Buffer::default(), 
            need_redraw: true, 
            size: Size::default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}

impl View {

    pub fn render_line(row: usize, line_text: &str) {
        if let Err(err) = Terminal::print_line(row, line_text) {
            eprintln!("Fail to Render: {err}");
        }
    }

/// Draws the rows of the editor on the terminal screen.
/// 
/// `truncated_line` is **NOT VERY SAFE**
    pub fn render(&mut self) {
        if !self.need_redraw {
            return ;
        }
        let Size{height, width} = self.size;
        let top = self.scroll_offset.row;

        for current_row in 0..height {
            //truncate line
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {

                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);

                let truncated_line = line.get_visible_graphemes(left..right);
                Self::render_line(current_row, truncated_line.as_str());

            }else {
                Self::render_line(current_row, "~");
            }
        }

        self.need_redraw = false;

    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => 
                self.resize(size),
            EditorCommand::Move(direction) => 
                self.move_text_location(&direction),
            EditorCommand::Quit => (),
        }
    }

    #[allow(dead_code)]
    fn draw_version() -> Result<(), Box<dyn Error>> {
        let version = env!("CARGO_PKG_VERSION");
        let name = "ZenQuill";
        let merrage = format!("{name} Editor v{version}");

        let width = Terminal::get_size()?.width as usize;
        let len = merrage.len();
        let padding = if width > len { (width - len) / 2 } else { 0 };
        let spaces = " ".repeat(padding.saturating_sub(1));
        let mut version_message = format!("~{spaces}{merrage}");

        version_message.truncate(width);

        Terminal::print(version_message.as_str())?;
        
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }

    pub fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines
                         .get(row)
                         .map_or(0, |line| {
                            line.width_until(self.text_location.grapheme_index)
                         });
        Position { col, row }
    }

    pub fn move_text_location(&mut self, direction: &Direction) {
        match direction {
            Direction::Up        =>  self.move_up(),
            Direction::Down      =>  self.move_down(),
            Direction::PageUp    =>  self.move_page_up(),
            Direction::PageDown  =>  self.move_page_down(),
            Direction::Left      =>  self.move_left(),
            Direction::Right     =>  self.move_right(),
            Direction::Home      =>  self.move_home(),
            Direction::End       =>  self.move_end(),
        }
        self.scroll_location_into_view()
    }

    fn move_up(&mut self) {
        self.text_location.line_index = self
            .text_location
            .line_index
            .saturating_sub(1);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self) {
        self.text_location.line_index = self
            .text_location
            .line_index
            .saturating_add(1);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_page_up(&mut self) {
        let Size{ height, .. } = self.size;
        self.text_location.line_index = self
            .text_location
            .line_index
            .saturating_sub(height);
        self.snap_to_valid_grapheme();
    }

    fn move_page_down(&mut self) {
        let Size{ height, .. } = self.size;
        self.text_location.line_index = self
            .text_location
            .line_index
            .saturating_add(height);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index == 0 {
            if self.text_location.line_index == 0 {
                return ;
            }
            self.move_up();
            self.move_end();
        } else {
            self.text_location.grapheme_index -= 1;
        }
    }

    fn move_right(&mut self) {
        let line_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.len());
        if self.text_location.grapheme_index >= line_len {
            self.move_down();
            self.move_home();
        } else {
            self.text_location.grapheme_index += 1;
        }
    }

    fn move_home(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_end(&mut self) {
        let line_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.len());
        self.text_location.grapheme_index = line_len;
    }

    // Ensures self.location.grapheme_index points to a valid grapheme index by snapping it to the left most grapheme if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                std::cmp::min(
                    line.grapheme_count(),
                    self.text_location.grapheme_index,
                )
            });
    }
    // Ensures self.location.line_index points to a valid line index by snapping it to the bottom most line if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = std::cmp::min(
            self.text_location.line_index, 
            self.buffer.height(),
        );
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            self.need_redraw = true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            self.need_redraw = true
        } 
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            self.need_redraw = true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            self.need_redraw = true
        }
    }

    pub fn scroll_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();

        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    pub fn cursor_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }
}