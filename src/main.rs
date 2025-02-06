use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod ui;

fn main() {
    const RESET: &str = "\x1b[0m";
    const HIGHLIGHT: &str = "\x1b[48;5;8m";
    println!("{}{}{}",HIGHLIGHT,"[X]", RESET);
    ui::GameField::init();

    fn capture_example () {

        // Enable raw mode to capture key events directly
        enable_raw_mode()?;

        loop {
            // Wait for a key event
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => println!("Up arrow pressed"),
                    KeyCode::Down => println!("Down arrow pressed"),
                    KeyCode::Left => println!("Left arrow pressed"),
                    KeyCode::Right => println!("Right arrow pressed"),
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Disable raw mode before exiting
        disable_raw_mode()?;
        Ok(()).expect("TODO: panic message");
    }
}
