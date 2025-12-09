use crossterm::cursor::{ Hide, Show, MoveTo };
use crossterm::terminal::{ Clear, ClearType };

use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use crossterm::{ queue, Command };

use std::io::{stdout, Write};
use std::error::Error;

// use super::cursor::Location;
use crate::core::Location;

pub struct Terminal;

/// Represents the size of the terminal window.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Size{
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub fn new() -> Self {
        Terminal::get_size().unwrap_or_default()
    }
}

/// Represents a position in the terminal window.
#[derive(Clone, Copy, Default)]
pub struct Position{
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Position { row: x, col: y }
    }

    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}


impl From<Location> for Position {
    fn from(value: Location) -> Self {
        Self { row: value.line_index, col: value.grapheme_index }
    }
}

impl Terminal {
/// Initializes the terminal.
/// 
/// Enables raw mode, clears the screen, and moves the cursor to the top-left corner.
    pub fn initialize() -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::new(0, 0))?;
        Self::execute()?;
        Ok(())
    }

/// Terminates the terminal.
/// 
/// Clears the screen and disables raw mode.
    pub fn terminate() -> Result<(), Box<dyn Error>> {
        Self::leave_alternate_screen()?;
        Self::clear_screen()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Box<dyn Error>> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Box<dyn Error>> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    // Queues a terminal command for execution.
    fn queue_command(command: impl Command) -> Result<(), Box<dyn Error>> {
        queue!(stdout(), command)?;
        Ok(())
    }

/// Clears the terminal screen.
    pub fn clear_screen() -> Result<(), Box<dyn Error>> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

/// Clears current line
    pub fn clear_line() -> Result<(), Box<dyn Error>> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

/// Hides the terminal caret.
/// 
/// Returns an error if the operation fails.
    pub fn hide_caret() -> Result<(), Box<dyn Error>> {
        Self::queue_command(Hide)?;
        Ok(())
    }

/// Shows the terminal cursor.
/// 
/// Returns an error if the operation fails.
    pub fn show_caret() -> Result<(), Box<dyn Error>> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Box<dyn Error>> {
        queue!(stdout(), MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    /// Prints a string to the terminal.
    /// 
    /// Returns an error if the operation fails.
    pub fn print(string: &str) -> Result<(), Box<dyn Error>> {
        Self::queue_command(crossterm::style::Print(string))?;
        Ok(())
    }

    pub fn print_line(row: usize, line_text: &str) -> Result<(), Box<dyn Error>> {
        Self::move_cursor_to(Position::new(row, 0))?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    /// Flushes the queued terminal commands to the terminal.
    /// 
    /// Returns an error if the operation fails.
    pub fn execute() -> Result<(), Box<dyn Error>> {
        stdout().flush()?;
        Ok(())
    }

    pub fn get_size() -> Result<Size, Box<dyn Error>> {
        let (width, height) = crossterm::terminal::size()?;
        let width = width as usize;
        let height = height as usize;
        Ok(Size { height, width })
    }
}