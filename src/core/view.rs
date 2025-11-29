use std::error::Error;

use crate::core::Position;
use crate::core::command::{Direction, EditorCommand};
use crate::core::cursor::Location;

use super::terminal::{ Size, Terminal };
use super::buffer::Buffer;

pub struct View{
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        View { 
            buffer: Buffer::default(), 
            need_redraw: true, 
            size: Size::default(),
            location: Location::default(),
            scroll_offset: Location::default(),
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
        let top = self.scroll_offset.y;

        for current_row in 0..height {
            //truncate line
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {

                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);

                let truncated_line = line.get(left..right);
                Self::render_line(current_row, truncated_line);

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

    pub fn load(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
        Ok(())
    }

    pub fn move_text_location(&mut self, direction: &Direction) {
        let Location {mut x, mut y} = self.location;
        let Size{ height, width } = self.size;

        let h = height;
        let w = width;

        match direction {
            Direction::Up           =>  y = y.saturating_sub(1),
            Direction::Left         =>  x = x.saturating_sub(1),
            Direction::Down         =>  y = y.saturating_add(1),
            Direction::Right        =>  x = x.saturating_add(1),
            Direction::PageUp       =>  y = 0,
            Direction::PageDown     =>  y = h.saturating_sub(1),
            Direction::Home         =>  x = 0,
            Direction::End          =>  x = w.saturating_sub(1),
        }
        self.location = Location { x, y };
        self.scroll_location_into_view()
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    pub fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { height, width } = self.size;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            self.need_redraw = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            self.need_redraw = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            self.need_redraw = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            self.need_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        Position::new(
            self.location.y.saturating_sub(self.scroll_offset.y),
            self.location.x.saturating_sub(self.scroll_offset.x),
        )
    }
}