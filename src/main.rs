mod editor;
use editor::Editor;

// We want to get RAW MODE and not CANONICAL MODE

fn main() {
  let editor = Editor::default();
  editor.run();
}
