
use std::io::{self, stdout, Write};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
  pub width: u16,
  pub height: u16,
}

pub struct Terminal {
  size: Size,
  _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
  pub fn default() -> Result<Self, std::io::Error> {
    let size = termion::terminal_size()?;
    Ok(Self {
      size: Size {
        width: size.0,
        height: size.1,
      },
      _stdout: stdout().into_raw_mode()?,
    })
  }
  
  pub fn size(&self) -> &Size {
    &self.size
  }
  
  pub fn read_key() -> Result<Key, std::io::Error> {
    loop {
      // io::stin().lock().keys().next() returns a Option<Result<key, err>>
      // we unwrap Option in the next line and we unwrap Result in line 32
      if let Some(key) = io::stdin().lock().keys().next() {
        return key;
      }
    }
  }
  
  pub fn clear_screen() {
    print!("{}", termion::clear::All);
  }
  
  pub fn cursor_position(x: u16, y: u16) {
    // saturating_add just add 1 to x or y
    // this way cursor position on screen is 0-base and
    // not 1-base
    // saturating_add try to add a 1 but if the max value is
    // is reached is does not go in overflow and it returns the max
    // value available
    let x = x.saturating_add(1);
    let y = y.saturating_add(1);
    print!("{}", termion::cursor::Goto(x, y));
  }
  
  pub fn flush() -> Result<(), std::io::Error> {
    io::stdout().flush()
  }
}