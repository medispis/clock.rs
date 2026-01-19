extern crate chrono;
use crate::event::Event;
use chrono::prelude::*;
use clap::Parser;
use crossterm::event::{self, KeyCode};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "clock")]
    mode: String,
    #[arg(short)]
    second: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            let clock = get_time();
            let block_time = get_block_text(&clock);

            let paragraph = Paragraph::new(block_time)
                .centered()
                .block(Block::default().borders(Borders::NONE));

            terminal.draw(|frame| frame.render_widget(paragraph, frame.area()))?;
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
    let args = Args::parse();
    let utc: DateTime<Local> = Local::now();
    match (args.mode.as_str(), args.second) {
        ("clock", true)  => utc.format("%H:%M:%S").to_string(),
        ("clock", false) => utc.format("%H:%M").to_string(),
        ("date", _)      => utc.format("%Y-%m-%d").to_string(),
        _                => utc.format("%H:%M:%S").to_string(),
    }
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
    font.insert('-', vec!["      ", "      ", "██████", "      ", "      "]);
    font.insert(' ', vec!["      ", "      ", "      ", "      ", "      "]);

    let mut result = String::new();

    for i in 0..5 {
        for c in text.chars() {
            if let Some(lines) = font.get(&c) {
                result.push_str(lines[i]);
                result.push(' ');
            } else {
                result.push_str("      ");
                result.push(' ');
            }
        }
        result.push('\n');
    }

    result
}
