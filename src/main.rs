#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;

// We want to get RAW MODE and not CANONICAL MODE

fn main() {
  // let indicates that editor is a read-only reference, so we can't
  // execute run on it because it mutates editor
  // let editor = Editor::default();
  Editor::default().run();
}
