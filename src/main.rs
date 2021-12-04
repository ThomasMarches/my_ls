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
    if std::env::args().len() == 3 {
        let second_arg = match std::env::args().nth(2) {
            Some(it) => it,
            None => {
                show_usage();
                return;
            }
        };
        let arg = match std::env::args().nth(1) {
            Some(it) => it,
            None => {
                show_usage();
                return;
            }
        };
        if second_arg == "-R" {
            tool::process_folders(&arg, true);
        } else {
            show_usage();
        }
    }

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
    tool::process_folders(&arg, false);
}
