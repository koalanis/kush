use crate::kush;

use std::io::{self, Write};

pub fn shell_init() {
  let _ = io::stdout().flush();
  println!("shell starting - welcome!")
}


pub fn shell_loop() {
  loop {
    let user_input = kush::prompt();
    let status = kush::parse_cmd(user_input);
    let _ = io::stdout().flush();
    if status == kush::ShellStatus::ExitInvoked {
      break;
    }
  }
}

