extern crate chrono;
use crate::event::Event;
use chrono::prelude::*;
use crossterm::event::{self, KeyCode};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            let clock = get_time();
            terminal.draw(|frame| frame.render_widget(clock, frame.area()))?;

            // Check for key events with a short timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('c') {
                        break;
                    }
                }
            }
        }
        Ok(())
    })
}
fn get_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.format("%H:%M:%S").to_string()
}
