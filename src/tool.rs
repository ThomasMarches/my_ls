use std::{fs, str::Lines};

use crate::FolderResult;

fn is_file_a_coding_file(file_name: &str) -> bool {
    let file_extension = file_name.split(".").last().unwrap();
    return file_extension == "rs"
        || file_extension == "c"
        || file_extension == "js"
        || file_extension == "cpp"
        || file_extension == "hs"
        || file_extension == "java"
        || file_extension == "asm"
        || file_extension == "py"
        || file_extension == "cxx"
        || file_extension == "h"
        || file_extension == "hpp"
        || file_extension == "hxx"
        || file_extension == "html"
        || file_extension == "css"
        || file_extension == "dart"
        || file_extension == "jsx"
        || file_extension == "json";
}

pub fn get_files_names(folder: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let tmp = path.unwrap().path();
        let str = tmp.to_str();
        let string = str.map(|s| s.to_string());
        if tmp.is_dir() {
            result.append(&mut get_files_names(&string.unwrap()));
        } else {
            let changed_string = string.unwrap().replace("./", "");
            if changed_string.starts_with(".") {
                continue;
            }
            result.push(changed_string);
        }
    }
    result
}

pub fn process_file(path: &str, lines: &Lines, result: &mut FolderResult) {
    result.code_file_number += 1;
    let coding_file = is_file_a_coding_file(path);

    lines.to_owned().for_each(|line| {
        let final_line = line.replace(" ", "");
        if line == "" {
            if coding_file {
                result.code_lines -= 1;
            }
            result.empty_lines += 1;
        } else if final_line.starts_with("//")
            || final_line.starts_with("/*")
            || final_line.starts_with("#")
        {
            if coding_file {
                result.code_lines -= 1;
            }
            result.comment_lines += 1;
        }
        if coding_file {
            result.code_lines += 1;
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
        let mut result = FolderResult {
            file_number: 0,
            code_file_number: 0,
            comment_lines: 0,
            code_lines: 0,
            empty_lines: 0,
            lines: 0,
        };

        let path = "test_folder/test.rs";
        let file = fs::read_to_string(path);
        let unwraped_file = file.unwrap();
        let lines = unwraped_file.lines();

        process_file(path, &lines, &mut result);
        assert_eq!(result.empty_lines, 1);
        assert_eq!(result.comment_lines, 2);
        assert_eq!(result.code_lines, 3);
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
