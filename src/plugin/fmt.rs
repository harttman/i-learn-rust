use std::io::{self, Write};

pub struct Console {}
impl Console {
  pub fn log<T:std::fmt::Display>(text: T) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", text)
      .expect("Error");
    handle.flush()
      .expect("Error");
  }
}
