/* equivalent of 
use std::io;
use std::io::Read;
*/
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

// We want to get RAW MODE and not CANONICAL MODE

fn to_ctrl_byte(c: char) -> u8 {
  let byte = c as u8;
  byte & 0b0001_1111
}

fn die(e: std::io::Error) {
  panic!("{}", e);
}

fn main() {
  // with _ we tell others that we want to hold on to _stdout
  // even though we are not using it -> without compiler throw warning.
  // it's necessary to assign to variable otherwise terminal
  // wont' stay in RAW mode
  let _stdout = stdout().into_raw_mode().unwrap();

  for b in io::stdin().bytes() {
    match b {
      Ok(b) => {
        let c = b as char;
        
        // is control checks whether the character is a control character
        // which are non-printable characters. (0 - 31 and 127) are non printable
        if c.is_control() {
          println!("{:?} \r", b);
        } else {
          println!("{:?} ({})\r", b, c);
        }
        // {} in println! is for elements for which a printable representation
        // is known such as char, while {:?} is a placeholder for elements
        // for which a string representation is not known bug a "debug representation"
          
        // control that ctrl + q is pressed
        if b == to_ctrl_byte('q') {
          break;
        }
      }
      Err(err) => die(err),
    }
  }
}
