/* equivalent of 
use std::io;
use std::io::Read;
*/
use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/* The reason why Position is written here and not in terminal
is because the cursor of the terminal is different from the cursor 
of the document we are currently editing. So they don't have to match
and more importantly the are different cursors. */
pub struct Position {
  pub x: usize,
  pub y: usize,
}

pub struct Editor {
  should_quit: bool,
  terminal: Terminal,
  cursor_position: Position,
}

impl Editor {
  // &mut self instead of &self it's needed when you edit the struct
  // it must be used wherever you change the instance (editor in this case)
  pub fn run(&mut self) {
    // with _ we tell others that we want to hold on to _stdout
    // even though we are not using it -> without compiler throw warning.
    // it's necessary to assign to variable otherwise terminal
    // wont' stay in RAW mode
  
    loop {
      // if let is a shorthand for match, used when we want to 
      // catch just one condition and nothing else
      
      // called here to refresh the screen one last time after the user
      // decide to quit
      if let Err(err) = self.refresh_screen() {
        die(&err);
      }
           
      if self.should_quit {
        break;
      }
      
      if let Err(err) = self.process_keypress() {
        die(&err);
      }
      
    }
  }
  
  fn refresh_screen(&self) -> Result<(), std::io::Error> {
    // VT100 escape sequences -> this one is used to clear the screen
    // print!("\x1b[2J") // termion offer a shorthand;
    // if I don't put a ; after last instruction it means it is the 
    // return value
    // flush make sure that stdout print everything it has (in buffer)
    // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    Terminal::cursor_hide();
    Terminal::cursor_position(&Position { x: 0, y: 0 });
    if self.should_quit {
      Terminal::clear_screen();
      println!("Goodbye.\r");
    } else {
      self.draw_rows();
      
      // when I finished drawing ~ i put the cursor back to the top left
      // print!("{}", termion::cursor::Goto(1, 1));
      // Terminal::cursor_position(0, 0);
      Terminal::cursor_position(&self.cursor_position);
    }
    // io::stdout().flush()
    Terminal::cursor_show();
    Terminal::flush()
  }
  
  fn draw_welcome_message(&self) {
    // truncate part of the string that doesn't fit in the current terminal width size
    let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
    let width = self.terminal.size().width as usize;
    let len = welcome_message.len();
    let padding = width.saturating_sub(len) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    welcome_message = format!("~{}{}", spaces, welcome_message);
    welcome_message.truncate(width);
    println!("{}\r", welcome_message);
  }
  
  fn draw_rows(&self) {
    let height = self.terminal.size().height;
    for row in 0..height - 1 {
      Terminal::clear_current_line();
      if row == height / 3 {
        self.draw_welcome_message();
      } else {
        println!("~\r");
      }
    }
  }
  
  fn process_keypress(&mut self) -> Result<(), std::io::Error> {
    // '?' means if there's an error return it, if not, unwrap the value and continue
    // let pressed_key = read_key()?;
    let pressed_key = Terminal::read_key()?;
    match pressed_key {
      Key::Ctrl('q') => self.should_quit = true,
      Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
      _ => (),
    }
    
    // this mean Everything is OK and nothing is returned
    // rust does not have any try catch so this is the only
    // way we can tell the parent method that everyting is OK
    // in case of error it gets returned through the 2nd parameter
    // of Result<>
    Ok(())
  }
  
  fn move_cursor(&mut self, key: Key) {
    let Position { mut x, mut y } = self.cursor_position;
    match key {
      Key::Up => y = y.saturating_sub(1),
      Key::Down => y = y.saturating_add(1),
      Key::Left => x = x.saturating_sub(1),
      Key::Right => x = x.saturating_add(1),
      _ => (),
    }
    self.cursor_position = Position { x, y }
  }
    
  // this method create a default object so that we don't have to 
  // setup it on our own each time
  // when I don't specify &self as paramterer, it means that this is
  // a static method and can be called as Editor::default instead of 
  // usual dot notation (.method)
  pub fn default() -> Self {
    Self {
      should_quit: false,
      terminal: Terminal::default().expect("Failed to initialize terminal"),
      cursor_position: Position { x: 0, y: 0 }
    }
  }
}

fn die(e: &std::io::Error) {
  // print!("{}", termion::clear::All);
  Terminal::clear_screen();
  panic!("{}", *e);
}

