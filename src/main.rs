use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    println!("{:?} ({})\r", c as u8, c);
                }
                Key::Ctrl(c) => {
                    if c != 'q' {
                        println!("{:?}\r", c as u8);
                    } else {
                        break;
                    }
                }
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
}
