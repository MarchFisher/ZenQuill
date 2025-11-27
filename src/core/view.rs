use std::error::Error;

use super::terminal::{ Size, Terminal };

pub struct View;

impl View {

/// Draws the rows of the editor on the terminal screen.
    pub fn render() -> Result<(), Box<dyn Error>> {
        let Size{height, ..} = Terminal::get_size()?;
        for current_row in 0..height {
            if current_row == height / 3 * 2 {
                Self::draw_version()?;
            }else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    
    pub fn draw_empty_row() -> Result<(), Box<dyn Error>> {
        Terminal::print("~")?;
        Ok(())
    }

    #[allow(dead_code)]
    fn draw_version() -> Result<(), Box<dyn std::error::Error>> {
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
}