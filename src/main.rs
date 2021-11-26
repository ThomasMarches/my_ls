use std::{
    fmt::{Debug, Error, Formatter},
    fs::{self},
};

struct FolderResult {
    file_number: i32,
    code_file_number: i32,
    lines: i32,
    code_lines: i32,
}

impl Debug for FolderResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "1 - Files: {},\n2 - Code files: {},\n3 - Lines: {},\n4 - Code lines: {}\n",
            self.file_number, self.code_file_number, self.lines, self.code_lines
        )
    }
}

fn get_files_names(folder: &str) -> Vec<String> {
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

fn count_number_of_lines_and_folders(paths: &Vec<String>, result: &mut FolderResult) {
    for path in paths {
        let file = fs::read_to_string(path);
        if file.is_err() {
            continue;
        }
        let unwraped_file = file.unwrap();
        let lines = unwraped_file.lines();
        result.lines += lines.clone().count() as i32;

        lines.for_each(|line| {
            if line.contains("fn") {
                result.code_lines += 1;
            }
        });

        let file_extension = path.split(".").last().unwrap();

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
            result.code_file_number += 1;
        }
    }
}

fn main() {
    let pattern = std::env::args().nth(1);

    if pattern.is_none() || pattern == Some(String::from("-h")) {
        println!("{}", fs::read_to_string("help.txt").unwrap());
        return;
    }

    let mut result = FolderResult {
        file_number: 0,
        code_file_number: 0,
        code_lines: 0,
        lines: 0,
    };

    let paths = get_files_names(&pattern.unwrap());
    result.file_number = paths.len() as i32;
    count_number_of_lines_and_folders(&paths, &mut result);
    println!("{:?}", result);
}
