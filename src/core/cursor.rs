use std::cmp::min;

use crossterm::event::KeyCode;

use crate::core::Size;

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Cursor {
    location: Location,
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_cursor(&mut self, key_code: KeyCode) {
        let Location {mut x, mut y} = self.location;

        let Size{ height, width } = Size::new();
        let h = height;
        let w = width;

        match key_code {
            KeyCode::Up => 
                y = y.saturating_sub(1),
            KeyCode::Left =>
                x = x.saturating_sub(1),
            KeyCode::Down => 
                y = min(h, y.saturating_add(1)),
            KeyCode::Right =>
                x = min(w, x.saturating_add(1)),
            KeyCode::PageUp     =>  y = 0,
            KeyCode::PageDown   =>  y = h,
            KeyCode::Home       =>  x = 0,
            KeyCode::End        =>  x = w,
            _ => ()
        }
        self.location = Location { x, y };
    }

    pub fn get_col(self) -> usize {
        return self.location.x;
    }

    pub fn get_row(self) -> usize {
        return self.location.y;
    }
}