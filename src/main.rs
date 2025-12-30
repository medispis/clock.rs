extern crate chrono;
use std::thread;

use chrono::prelude::*;

fn main() {
    loop {
        let utc: DateTime<Utc> = Utc::now();
        thread::sleep(std::time::Duration::from_millis(1000));
        let clock = utc.format("%H:%M:%S");
        println!("{}", clock);
    }
}
