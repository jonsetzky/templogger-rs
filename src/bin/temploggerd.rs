use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::{thread, time};

#[path = "../util.rs"]
mod util;

/// Appends the log with a string
fn append_log(s: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(util::LOG_FILE)
        .unwrap();

    match writeln!(file, "{}", s) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn main() {
    if !util::validate_system() {
        eprintln!("System not supported!");
        return;
    }

    if !Path::new(util::LOG_DIR).exists() {
        fs::create_dir_all(util::LOG_DIR).expect("could not create log dir");
    };
    if !Path::new(util::LOG_FILE).exists() {
        fs::File::create(util::LOG_FILE).expect("could not create log file");
    };
    let mut i = 0;
    loop {
        append_log(format!(
            "{}{}",
            util::get_temp(),
            if i < 1 { // print time every 100th log entry
                format!(", {}", util::get_time())
            } else {
                String::from("")
            }
        ));

        i = (i + 1) % 100;

        thread::sleep(time::Duration::from_secs(30));
    }
}
