#![warn(clippy::all, clippy::pedantic)]
use zen_quill::Editor;

fn main() {
    let mut editor = Editor::new();
    if let Err(err) = editor.run() {
        eprintln!("Application error: {err}");
    }
}