use std::fs;
use crate::Row;

#[derive(Default)] // -> compiler will try to derive default values for our struct
// we don't need to implement ::default anymore
// rust will 'guess' the corret type to initialize the struct
// for complex struct we can't use it because data is too complex, we can use instead
// method ::default()
pub struct Document {
  rows: Vec<Row>,
}

impl Document {
  pub fn open(filename: &str) -> Result<Self, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut rows = Vec::new();
    // rows.push(Row::from("Hello World"));
    // Self { rows }
    for value in contents.lines() {
      rows.push(Row::from(value));
    }
    Ok(Self { rows })
  }
  
  pub fn row(&self, index: usize) -> Option<&Row> {
    self.rows.get(index)
  }
  
  pub fn is_empty(&self) -> bool {
    self.rows.is_empty()
  }
  
  pub fn len(&self) -> usize {
    self.rows.len()
  }
}
