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
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name)?;
        }

        Ok(Self { should_quit: false, cursor: Cursor::default(), view })
    }
    
/// The Read-Eval-Print Loop (REPL) for the editor.
    pub fn run(&mut self) {
        loop {
            // Refresh the screen
            // Check if we should quit
            // Read an event
            // Evaluate the event

            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    eprint!("Could not read event: {err:?}");
                }
            }
        }
    }

/// Refreshes the terminal screen based on the current editor state.
    fn refresh_screen(&mut self) {
        // Hide the cursor to prevent flickering during updates
        let _ = Terminal::hide_caret();
        let _ = Terminal::move_cursor_to(Position::default());

        // Check if we should quit
        if self.should_quit {
            let _ = Terminal::clear_screen();
            let _ = Terminal::print("Goodbye!\r\n");
        }else {
            // Draw the rows
            let _ = self.view.render();
            // Self::draw_version()?;
            let _ = Terminal::move_cursor_to(Position::new(
                self.cursor.get_row(),
                self.cursor.get_col()
            ));
        }

        // Show the cursor again after updates
        // Execute all terminal commands
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

/// Evaluates a key event and updates the editor state accordingly.
    fn evaluate_event(&mut self, event: &Event) {
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
                    self.cursor.move_cursor(*code),
                _ => (),
            }
        }else if let Event::Resize(width, height) =  event{
            let width = *width as usize;
            let height = *height as usize;
            self.view.resize(Size { height, width });
        }
    }


}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}