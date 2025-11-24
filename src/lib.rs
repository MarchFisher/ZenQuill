use crossterm::event::{
    Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read
};

/// The main text editor structure,
/// responsible for managing the editor state and user interactions.
mod terminal;
use terminal::{ Terminal, Size, Position, Caret };

/// Represents the main text editor.
#[derive(Default)]
pub struct Editor{
    should_quit: bool,

    pub caret: Caret
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
            Self::draw_rows()?;
            // Self::draw_version()?;
            Terminal::move_caret_to(Position::new(
                self.caret.get_col(),
                self.caret.get_row()
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
                    self.caret.move_caret(*code)?,
                _ => (),
            }
        }
        Ok(())
    }

/// Draws the rows of the editor on the terminal screen.
    fn draw_rows() -> Result<(), Box<dyn std::error::Error>> {
        let Size{height, ..} = Terminal::get_size()?;
        for current_row in 0..height {
            Terminal::print("~")?;
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn draw_version() -> Result<(), Box<dyn std::error::Error>> {
        let version = env!("CARGO_PKG_VERSION");
        let name = "ZenQuill";
        let merrage = format!("{name} Editor v{version}");

        let Size{height, width} = Terminal::get_size()?;
        let width = width as usize;
        let len = merrage.len();
        let padding = if width > len { (width - len) / 2 } else { 0 };
        let spaces = " ".repeat(padding.saturating_sub(1));
        let mut version_message = format!("~{spaces}{merrage}{spaces}");

        version_message.truncate(width);

        Terminal::move_caret_to(Position::new(0, height / 3 * 2))?;
        Terminal::print(version_message.as_str())?;
        
        Ok(())
    }

}