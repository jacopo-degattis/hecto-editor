/* equivalent of 
use std::io;
use std::io::Read;
*/
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
  should_quit: bool,
}

impl Editor {
  // &mut self instead of &self it's needed when you edit the struct
  // it must be used wherever you change the instance (editor in this case)
  pub fn run(&mut self) {
    // with _ we tell others that we want to hold on to _stdout
    // even though we are not using it -> without compiler throw warning.
    // it's necessary to assign to variable otherwise terminal
    // wont' stay in RAW mode
    let _stdout = stdout().into_raw_mode().unwrap();
  
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
    print!("{}", termion::clear::All);
    io::stdout().flush()
  }
  
  fn process_keypress(&mut self) -> Result<(), std::io::Error> {
    // '?' means if there's an error return it, if not, unwrap the value and continue
    let pressed_key = read_key()?;
    match pressed_key {
      Key::Ctrl('q') => self.should_quit = true,
      _ => (),
    }
    
    // this mean Everything is OK and nothing is returned
    // rust does not have any try catch so this is the only
    // way we can tell the parent method that everyting is OK
    // in case of error it gets returned through the 2nd parameter
    // of Result<>
    Ok(())
  }
    
  // this method create a default object so that we don't have to 
  // setup it on our own each time
  // when I don't specify &self as paramterer, it means that this is
  // a static method and can be called as Editor::default instead of 
  // usual dot notation (.method)
  pub fn default() -> Self {
    Self { should_quit: false }
  }
}

// TODO: move inside of impl (class) ?
fn read_key() -> Result<Key, std::io::Error> {
  loop {
    // io::stin().lock().keys().next() returns a Option<Result<key, err>>
    // we unwrap Option in the next line and we unwrap Result in line 32
    if let Some(key) = io::stdin().lock().keys().next() {
      return key;
    }
  }
}

fn die(e: &std::io::Error) {
  panic!("{}", *e);
}

