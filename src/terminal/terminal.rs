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
    pub height: u16,
    pub width: u16,
}

/// Represents a position in the terminal window.
#[derive(Clone, Copy)]
pub struct Position{
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub const fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }
}

impl Terminal {
    /// Initializes the terminal.
    /// 
    /// Enables raw mode, clears the screen, and moves the cursor to the top-left corner.
    pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::new(0, 0))?;
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

    /// Hides the terminal cursor.
    /// 
    /// Returns an error if the operation fails.
    pub fn hide_cursor() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Shows the terminal cursor.
    /// 
    /// Returns an error if the operation fails.
    pub fn show_cursor() -> Result<(), Box<dyn std::error::Error>> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Box<dyn std::error::Error>> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
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
        Ok(Size { height, width })
    }
}