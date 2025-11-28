use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read
};

/// The main text editor structure,
/// responsible for managing the editor state and user interactions.
mod core;
use core::{ Terminal, Size, View, Position, Cursor };

use std::error::Error;

/// Represents the main text editor.
#[derive(Default)]
pub struct Editor{
    should_quit: bool,

    pub cursor: Cursor,
    pub view: View
}

impl Editor {
/// Creates a new instance of the `Editor`.
    pub fn new() -> Self {
        Self::default()
    }

/// Runs the main loop of the editor.
/// 
/// ### Parameter:
/// None
/// 
/// ### Return Value:
/// a `Result` indicating success or failure.
/// 
/// ### Steps:
/// - Initializes the terminal
/// - Starts the REPL
/// - Terminates the terminal on exit.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Terminal::initialize()?;
        self.handle_args()?;
        let result = self.repl();
        Terminal::terminate()?;
        result
    }

    fn handle_args(&mut self) -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name)?;
        }
        Ok(())
    }
    
/// The Read-Eval-Print Loop (REPL) for the editor.
    fn repl(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // Refresh the screen
            // Check if we should quit
            // Read an event
            // Evaluate the event

            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

/// Refreshes the terminal screen based on the current editor state.
    fn refresh_screen(&mut self) -> Result<(), Box<dyn Error>> {
        // Hide the cursor to prevent flickering during updates
        Terminal::hide_caret()?;
        Terminal::move_cursor_to(Position::default())?;

        // Check if we should quit
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        }else {
            // Draw the rows
            self.view.render()?;
            // Self::draw_version()?;
            Terminal::move_cursor_to(Position::new(
                self.cursor.get_row(),
                self.cursor.get_col()
            ))?;
        }

        // Show the cursor again after updates
        // Execute all terminal commands
        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

/// Evaluates a key event and updates the editor state accordingly.
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Box<dyn Error>>{
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char('z') if *modifiers == KeyModifiers::CONTROL => 
                    self.should_quit = true,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::Home
                | KeyCode::End =>
                    self.cursor.move_cursor(*code)?,
                _ => (),
            }
        }else if let Event::Resize(width, height) =  event{
            let width = *width as usize;
            let height = *height as usize;
            self.view.resize(Size { height, width });
        }
        Ok(())
    }


}