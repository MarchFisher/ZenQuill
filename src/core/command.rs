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
    Insert(char),
    Backspace,
    Delete,
    Tab,
    Enter,
    Save,
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
                match (code, *modifiers) {
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),

                    (
                        KeyCode::Char(character), 
                        KeyModifiers::NONE | 
                        KeyModifiers::SHIFT,
                    ) => Ok(Self::Insert(*character)),

                    (KeyCode::Tab     , KeyModifiers::NONE) => Ok(Self::Tab),
                    (KeyCode::Enter   , KeyModifiers::NONE) => Ok(Self::Enter),

                    (KeyCode::Up      , KeyModifiers::NONE) => Ok(Self::Move(Direction::Up      )),
                    (KeyCode::Down    , KeyModifiers::NONE) => Ok(Self::Move(Direction::Down    )),
                    (KeyCode::Left    , KeyModifiers::NONE) => Ok(Self::Move(Direction::Left    )),
                    (KeyCode::Right   , KeyModifiers::NONE) => Ok(Self::Move(Direction::Right   )),
                    (KeyCode::PageUp  , KeyModifiers::NONE) => Ok(Self::Move(Direction::PageUp  )),
                    (KeyCode::PageDown, KeyModifiers::NONE) => Ok(Self::Move(Direction::PageDown)),
                    (KeyCode::Home    , KeyModifiers::NONE) => Ok(Self::Move(Direction::Home    )),
                    (KeyCode::End     , KeyModifiers::NONE) => Ok(Self::Move(Direction::End     )),

                    (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::Backspace),
                    (KeyCode::Delete   , KeyModifiers::NONE) => Ok(Self::Delete),
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
