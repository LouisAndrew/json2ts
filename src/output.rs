use std::io::{self, StdoutLock, Write};

pub trait Output {
    fn write(&mut self, content: &str);
}

pub struct ConsoleOutput {
    handle: StdoutLock<'static>,
}

impl Output for ConsoleOutput {
    fn write(&mut self, content: &str) {
        match writeln!(self.handle, "{}", content) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error whle printing: {}", error);
            }
        }
    }
}

pub fn get_output_channel() -> Box<dyn Output> {
    let stdout = io::stdout();
    let handle = stdout.lock();

    Box::new(ConsoleOutput { handle })
}
