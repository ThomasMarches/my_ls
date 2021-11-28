use std::{fs, str::Lines};

mod folder;

fn is_file_a_coding_file(file_name: &str) -> bool {
    match file_name.split(".").last() {
        Some(extension) => {
            return extension == "rs"
                || extension == "c"
                || extension == "js"
                || extension == "cpp"
                || extension == "hs"
                || extension == "java"
                || extension == "asm"
                || extension == "py"
                || extension == "cxx"
                || extension == "h"
                || extension == "hpp"
                || extension == "hxx"
                || extension == "html"
                || extension == "css"
                || extension == "dart"
                || extension == "jsx"
                || extension == "json"
        }
        None => return false,
    }
}

pub fn process_folders(paths: &Vec<String>) {
    let mut result = folder::FolderResult::default();

    for path in paths {
        let buffer = match fs::read_to_string(path) {
            Ok(buffer) => buffer,
            Err(_) => continue,
        };
        let lines = buffer.lines();
        result.increment_lines(lines.to_owned().count() as i32);
        result.increment_file_number();
        process_file(path, &lines, &mut result);
    }
    println!("{:?}", result);
}

pub fn get_files_names(folder: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let paths = match fs::read_dir(folder) {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("[Error handler]: {}.", err);
            return result;
        }
    };

    for path in paths {
        match path {
            Ok(path) => {
                let filepath = path.path();
                let str = filepath.to_str();
                let string = match str.map(|s| s.to_string()) {
                    Some(string) => string,
                    None => continue,
                };
                if filepath.is_dir() {
                    result.append(&mut get_files_names(&string));
                } else {
                    let changed_string = string.replace("./", "");
                    if changed_string.starts_with(".") {
                        continue;
                    }
                    result.push(changed_string);
                }
            }
            Err(err) => {
                eprintln!("[Error handler]: {}.", err);
                continue;
            }
        }
    }
    result
}

pub fn process_file(path: &str, lines: &Lines, result: &mut folder::FolderResult) {
    let coding_file = is_file_a_coding_file(path);

    if coding_file {
        result.increment_code_file_number();
    }

    lines.to_owned().for_each(|line| {
        let final_line = line.replace(" ", "");
        if line == "" {
            if coding_file {
                result.decrement_code_lines();
            }
            result.increment_empty_lines();
        } else if final_line.starts_with("//")
            || final_line.starts_with("/*")
            || final_line.starts_with("#")
        {
            if coding_file {
                result.decrement_code_lines();
            }
            result.increment_comment_lines();
        }
        if coding_file {
            result.increment_code_lines();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files_names() {
        let result = get_files_names("test_folder");
        assert_eq!(result, vec!["test_folder/test.rs"]);
    }

    #[test]
    fn test_process_file() {
        let mut result = folder::FolderResult::default();

        let path = "test_folder/test.rs";
        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("[Error handler]: {}.", err);
                return;
            }
        };
        let lines = file.lines();

        process_file(path, &lines, &mut result);
        assert_eq!(result.get_empty_lines(), 1);
        assert_eq!(result.get_comment_lines(), 2);
        assert_eq!(result.get_code_lines(), 3);
        assert_eq!(result.get_file_number(), 0);
        assert_eq!(result.get_code_file_number(), 1);
        assert_eq!(result.get_lines(), 0);
    }

    #[test]
    fn test_is_file_a_coding_file_with_proper_file() {
        assert_eq!(is_file_a_coding_file("main.rs"), true);
        assert_eq!(is_file_a_coding_file("main.c"), true);
        assert_eq!(is_file_a_coding_file("main.js"), true);
        assert_eq!(is_file_a_coding_file("main.cpp"), true);
        assert_eq!(is_file_a_coding_file("main.hs"), true);
        assert_eq!(is_file_a_coding_file("main.java"), true);
        assert_eq!(is_file_a_coding_file("main.asm"), true);
        assert_eq!(is_file_a_coding_file("main.py"), true);
        assert_eq!(is_file_a_coding_file("main.cxx"), true);
        assert_eq!(is_file_a_coding_file("main.h"), true);
        assert_eq!(is_file_a_coding_file("main.hpp"), true);
        assert_eq!(is_file_a_coding_file("main.hxx"), true);
        assert_eq!(is_file_a_coding_file("main.html"), true);
        assert_eq!(is_file_a_coding_file("main.css"), true);
        assert_eq!(is_file_a_coding_file("main.dart"), true);
        assert_eq!(is_file_a_coding_file("main.jsx"), true);
        assert_eq!(is_file_a_coding_file("main.json"), true);
    }

    #[test]
    fn test_is_file_a_coding_file_with_wrong_file() {
        assert_eq!(is_file_a_coding_file("main.txt"), false);
    }
}
