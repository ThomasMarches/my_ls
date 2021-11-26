use std::{fs, str::Lines};

pub fn is_file_a_coding_file(file_name: &str) -> bool {
    let file_extension = file_name.split(".").last().unwrap();
    if file_extension == "rs"
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
    {
        return true;
    }
    return false;
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
            result.push(string.unwrap().replace("./", ""));
        }
    }
    result
}

pub fn get_code_lines_number(lines: &Lines) -> i32 {
    let mut counter = 0;

    lines.to_owned().for_each(|line| {
        if line.contains("fn") {
            counter += 1;
        }
    });
    return counter;
}

pub fn get_comment_lines_number(lines: &Lines) -> i32 {
    let mut counter = 0;

    lines.to_owned().for_each(|line| {
        let final_line = line.replace(" ", "");
        if final_line.starts_with("//")
            || final_line.starts_with("/*")
            || final_line.starts_with("#")
        {
            counter += 1;
        }
    });
    return counter;
}
