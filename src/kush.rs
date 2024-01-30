use std::io::{self, BufRead, Write};

pub fn prompt() -> String {
  let mut stdout = io::stdout();
  print!("> ");
  let _ = stdout.flush();
  let mut line = String::new();

  let stdin = io::stdin();
  stdin.lock().read_line(&mut line).unwrap();
  return line;
}

#[derive(PartialEq)]
pub enum ShellStatus {
  ExitInvoked,
  Success,
  WaitOnProcess
}


fn exit() {
  println!("exiting kush...")
}

fn cmd_not_found(cmd: &str) {
  println!("cmd=`{}` not found", cmd)
}

pub fn parse_cmd(cmd: String) -> ShellStatus {
  let tokens: Vec<&str> = cmd.split_whitespace().collect();

  // TODO: parsing is more complex than checking for first token in cmd line
  // this needs to be enhanced to handle the entire kush scripting language spec
  // also TODO: come up with spec - im thinking of something functional, yet imperative
  // kind of making it up as a go, cuz im having too much fun just doing the regular coding part

  if tokens.len() == 0 {
    return ShellStatus::Success;
  }

  let first_token = tokens[0];
  let rest = &tokens[1..];
  
  match first_token {
    "help" => {help();  ShellStatus::Success},
    "echo" => {println!("{:?}", rest); ShellStatus::Success},
    "exit" => {exit(); ShellStatus::ExitInvoked},
    _ => {cmd_not_found(first_token); ShellStatus::Success}
  }

}

fn help() {

  println!("
kush - kaleb's unique shell
-----------------------------
echo - writes to stout 
exit - terminates shell session

  ")
}