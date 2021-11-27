use std::{
    fmt::{Debug, Error, Formatter},
    fs::{self},
};

mod tool;

pub struct FolderResult {
    file_number: i32,
    code_file_number: i32,
    comment_lines: i32,
    lines: i32,
    code_lines: i32,
    empty_lines: i32,
}

impl Default for FolderResult {
    fn default() -> Self {
        Self {
            file_number: 0,
            code_file_number: 0,
            comment_lines: 0,
            lines: 0,
            code_lines: 0,
            empty_lines: 0,
        }
    }
}

impl Debug for FolderResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "1 - Files: {},\n2 - Code files: {},\n3 - Lines: {},\n4 - Code lines: {},\n5 - Comment lines: {},\n6 - Blank lines: {}",
            self.file_number, self.code_file_number, self.lines, self.code_lines, self.comment_lines, self.empty_lines,
        )
    }
}

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

fn process_folders(paths: &Vec<String>, result: &mut FolderResult) {
    for path in paths {
        let buffer = match fs::read_to_string(path) {
            Ok(buffer) => buffer,
            Err(_) => continue,
        };
        let lines = buffer.lines();
        result.lines += lines.to_owned().count() as i32;
        result.file_number += 1;
        tool::process_file(path, &lines, result);
    }
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

    let mut result = FolderResult::default();
    let paths = tool::get_files_names(&arg);
    process_folders(&paths, &mut result);
    println!("{:?}", result);
}
