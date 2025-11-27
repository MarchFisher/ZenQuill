use crossterm::cursor::{ Hide, Show, MoveTo };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::{ queue, Command };
use std::fmt::Display;
use std::io::{stdout, Write};

pub struct Terminal;

/// Represents the size of the terminal window.
#[derive(Clone, Copy)]
pub struct Size{
    pub height: usize,
    pub width: usize,
}

/// Represents a position in the terminal window.
#[derive(Clone, Copy, Default)]
pub struct Position{
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Position { col: x, row: y }
    }
}

impl Terminal {
/// Initializes the terminal.
/// 
/// Enables raw mode, clears the screen, and moves the cursor to the top-left corner.
    pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position::new(0, 0))?;
        Self::execute()?;
        Ok(())
    }

/// Terminates the terminal.
/// 
/// Clears the screen and disables raw mode.
    pub fn terminate() -> Result<(), Box<dyn std::error::Error>> {
        Self::clear_screen()?;
        disable_raw_mode()?;
        Ok(())
    }

    // Queues a terminal command for execution.
    fn queue_command(command: impl Command) -> Result<(), Box<dyn std::error::Error>> {
        queue!(stdout(), command)?;
        Ok(())
    }

/// Clears the terminal screen.
    pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

/// Clears current line
    pub fn clear_line() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

/// Hides the terminal caret.
/// 
/// Returns an error if the operation fails.
    pub fn hide_caret() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Hide)?;
        Ok(())
    }

/// Shows the terminal cursor.
/// 
/// Returns an error if the operation fails.
    pub fn show_caret() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Box<dyn std::error::Error>> {
        queue!(stdout(), MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    /// Prints a string to the terminal.
    /// 
    /// Returns an error if the operation fails.
    pub fn print(string: impl Display) -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(crossterm::style::Print(string))?;
        Ok(())
    }

    /// Flushes the queued terminal commands to the terminal.
    /// 
    /// Returns an error if the operation fails.
    pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
        stdout().flush()?;
        Ok(())
    }

    pub fn get_size() -> Result<Size, Box<dyn std::error::Error>> {
        let (width, height) = crossterm::terminal::size()?;
        let width = width as usize;
        let height = height as usize;
        Ok(Size { height, width })
    }
}