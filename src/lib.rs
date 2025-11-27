use crossterm::event::{
    Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read
};

/// The main text editor structure,
/// responsible for managing the editor state and user interactions.
mod core;
use core::{ Terminal, View, Position, Cursor };

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
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Terminal::initialize()?;
        let result = self.repl();
        Terminal::terminate()?;
        result
    }
    
/// The Read-Eval-Print Loop (REPL) for the editor.
    fn repl(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
    fn refresh_screen(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Hide the cursor to prevent flickering during updates
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        // Check if we should quit
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        }else {
            // Draw the rows
            self.view.render()?;
            // Self::draw_version()?;
            Terminal::move_caret_to(Position::new(
                self.cursor.get_col(),
                self.cursor.get_row()
            ))?;
        }

        // Show the cursor again after updates
        // Execute all terminal commands
        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

/// Evaluates a key event and updates the editor state accordingly.
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>>{
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => 
                    self.should_quit = true,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::Home
                | KeyCode::End =>
                    self.cursor.move_caret(*code)?,
                _ => (),
            }
        }
        Ok(())
    }


}