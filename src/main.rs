#![warn(clippy::all, clippy::pedantic)]

mod document;
mod row;
mod editor;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

// We want to get RAW MODE and not CANONICAL MODE

fn main() {
  // let indicates that editor is a read-only reference, so we can't
  // execute run on it because it mutates editor
  // let editor = Editor::default();
  Editor::default().run();
}
