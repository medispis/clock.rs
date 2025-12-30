extern crate chrono;
use crate::event::Event;
use chrono::prelude::*;
use crossterm::event::{self, KeyCode};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            use ratatui::widgets::{Block, Borders, Paragraph};

            let clock = get_time();
            let block_time = get_block_text(&clock);

            let paragraph =
                Paragraph::new(block_time).block(Block::default().borders(Borders::NONE));

            terminal.draw(|frame| frame.render_widget(paragraph, frame.area()))?;

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
    let utc: DateTime<Local> = Local::now();
    utc.format("%H:%M:%S").to_string()
}

use std::collections::HashMap;

fn get_block_text(text: &str) -> String {
    let mut font: HashMap<char, Vec<&str>> = HashMap::new();

    font.insert('0', vec!["██████", "██  ██", "██  ██", "██  ██", "██████"]);
    font.insert('1', vec!["  ██  ", "  ██  ", "  ██  ", "  ██  ", "  ██  "]);
    font.insert('2', vec!["██████", "    ██", "██████", "██    ", "██████"]);
    font.insert('3', vec!["██████", "    ██", "██████", "    ██", "██████"]);
    font.insert('4', vec!["██  ██", "██  ██", "██████", "    ██", "    ██"]);
    font.insert('5', vec!["██████", "██    ", "██████", "    ██", "██████"]);
    font.insert('6', vec!["██████", "██    ", "██████", "██  ██", "██████"]);
    font.insert('7', vec!["██████", "    ██", "    ██", "    ██", "    ██"]);
    font.insert('8', vec!["██████", "██  ██", "██████", "██  ██", "██████"]);
    font.insert('9', vec!["██████", "██  ██", "██████", "    ██", "██████"]);

    font.insert(':', vec!["      ", "  ██  ", "      ", "  ██  ", "      "]);

    let mut result = String::new();

    for i in 0..5 {
        for c in text.chars() {
            if let Some(lines) = font.get(&c) {
                result.push_str(lines[i]);
                result.push(' '); // space between characters
            } else {
                result.push_str("      "); // blank for unknown characters
                result.push(' ');
            }
        }
        result.push('\n');
    }

    result
}
