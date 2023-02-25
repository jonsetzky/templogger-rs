use std::env;

#[path="../util.rs"]
mod util;

fn main() {
    if !util::validate_system() {
        eprintln!("System not supported!");
        return;
    }

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", util::get_temp());
    } else if args[1] == "log" {
        println!("{}", util::get_log(15));
    } else if args[1] == "version" {
        println!(env!("CARGO_PKG_VERSION"));
    } else {
        println!("Commands are: log");
    }
}
