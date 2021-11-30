use std::fs::{self};

mod tool;

fn show_usage() {
    let help_text = match fs::read_to_string("help.txt") {
        Ok(it) => it,
        Err(err) => {
            return eprintln!(
                "[Error handler]: {}.\nPlease check that help.txt file exists.",
                err
            )
        }
    };
    println!("{}", help_text);
}

fn main() {
    let arg = match std::env::args().nth(1) {
        Some(arg) => {
            if arg == "-h" || arg == "--help" {
                show_usage();
                return;
            }
            arg
        }
        None => {
            show_usage();
            return;
        }
    };
    tool::process_folders(arg);
}
