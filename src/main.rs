use std::process::ExitCode;
mod shell;
mod kush;
fn main() -> ExitCode {    
    shell::shell_init();
    shell::shell_loop();
    return ExitCode::SUCCESS;
}
