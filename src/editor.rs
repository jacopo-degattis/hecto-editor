/* equivalent of 
use std::io;
use std::io::Read;
*/
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
  pub fn run(&self) {
    // with _ we tell others that we want to hold on to _stdout
    // even though we are not using it -> without compiler throw warning.
    // it's necessary to assign to variable otherwise terminal
    // wont' stay in RAW mode
    let _stdout = stdout().into_raw_mode().unwrap();
  
    for key in io::stdin().keys() {
      match key {
        Ok(key) => match key {
          // matches any character and binds it to the variable c
          Key::Char(c) => {
            if c.is_control() {
              println!("{:?} \r", c as u8);
            } else {
              println!("{:?} ({})\r", c as u8, c);
            }
          }
          Key::Ctrl('q') => break,
          // default case
          _ => println!("{:?}\r", key),
        },
        Err(err) => die(&err),
      }
    }
  }
  
  // this method create a default object so that we don't have to 
  // setup it on our own each time
  // when I don't specify &self as paramterer, it means that this is
  // a static method and can be called as Editor::default instead of 
  // usual dot notation (.method)
  pub fn default() -> Self {
    Editor {}
  }
}

fn die(e: &std::io::Error) {
  panic!("{}", *e);
}

