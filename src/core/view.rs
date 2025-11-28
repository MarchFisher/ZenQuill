use std::error::Error;

use super::terminal::{ Size, Terminal };
use super::buffer::Buffer;

pub struct View{
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        View { 
            buffer: Buffer::default(), 
            need_redraw: true, 
            size: Size::default()
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
    pub fn render(&mut self) {
        if !self.need_redraw {
            return ;
        }
        let Size{height, width} = self.size;

        for current_row in 0..height {
            //truncate line
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = line
                    .get(0..width)
                    .unwrap_or(&line);
                Self::render_line(current_row, truncated_line);
            }else {
                Self::render_line(current_row, "~");
            }
        }

        self.need_redraw = false;

    }
    
    pub fn draw_empty_row() -> Result<(), Box<dyn Error>> {
        Terminal::print("~")?;
        Ok(())
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
        self.buffer = Buffer::load(file_name)?;
        Ok(())
    }

    pub fn resize(&mut self, new_size: Size) {
        self.need_redraw = true;
        self.size = new_size;
    }
}