#![warn(clippy::all, clippy::pedantic)]
use zen_quill::Editor;

fn main() {
    match Editor::new() {
        Ok(mut editor) => editor.run(),
        Err(err) =>
            eprintln!("Application error: {err}"),
    }
}