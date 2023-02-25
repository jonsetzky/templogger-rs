use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";
pub const LOG_DIR: &str = "/var/log/templogger";

#[cfg(not(debug_assertions))]
pub const LOG_FILE: &str = "/var/log/templogger/history-rs.log";
#[cfg(debug_assertions)]
pub const LOG_FILE: &str = "/var/log/templogger/history-rs-debug.log";

/// Returns the temperature of the rasppi processor
pub fn get_temp() -> f32 {
    let contents = fs::read_to_string(TEMP_PATH).expect("error reading temperature.");
    let contents: f32 = contents.trim().parse().expect("temp is not a float");
    contents / 1000.0
}

/// Returns the number of seconds since unix epoch
pub fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn get_log(n: usize) -> String {
    let contents = match fs::read_to_string(LOG_FILE) {
        Ok(content) => content,
        Err(e) => String::from("Log is empty"),
    };

    let mut out = String::from("");

    contents.lines().into_iter().rev().into_iter().take(n).into_iter().for_each(|line| {
        out.push_str(line);
        out.push_str("\n");
    });
    out
}

/// Validate that the system running is a raspbian
pub fn validate_system() -> bool {
    env::consts::ARCH == "aarch64" && env::consts::OS == "linux" && Path::new(TEMP_PATH).exists()
}
