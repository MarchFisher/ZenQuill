use crossterm::event::{ Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use super::terminal::Size;
use std::convert::TryFrom;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(PartialEq)]
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

impl TryFrom<&Event> for EditorCommand {
    type Error = String;

    fn try_from(event: &Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent { 
                code, 
                modifiers, 
                kind: KeyEventKind::Press, 
                .. 
            }) => {
                match code {
                    KeyCode::Char('z') if modifiers.contains(KeyModifiers::CONTROL) => Ok(Self::Quit),
                    KeyCode::Up         => Ok(Self::Move(Direction::Up      )),
                    KeyCode::Down       => Ok(Self::Move(Direction::Down    )),
                    KeyCode::Left       => Ok(Self::Move(Direction::Left    )),
                    KeyCode::Right      => Ok(Self::Move(Direction::Right   )),
                    KeyCode::PageUp     => Ok(Self::Move(Direction::PageUp  )),
                    KeyCode::PageDown   => Ok(Self::Move(Direction::PageDown)),
                    KeyCode::Home       => Ok(Self::Move(Direction::Home    )),
                    KeyCode::End        => Ok(Self::Move(Direction::End     )),
                    _ => Err(format!("Unsupported key: {:?}", code)),
                }
            }
            Event::Resize(width, height) => Ok(Self::Resize(Size {
                width: *width as usize,
                height: *height as usize,
            })),
            _ => Err(format!("Unsupported event: {:?}", event)),
        }
    }
}
