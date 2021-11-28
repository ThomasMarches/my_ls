use std::fmt;
use std::fmt::{Debug, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "1 - Files: {},\n2 - Code files: {},\n3 - Lines: {},\n4 - Code lines: {},\n5 - Comment lines: {},\n6 - Blank lines: {}",
            self.get_file_number(), self.get_code_file_number(), self.get_lines(), self.get_code_lines(), self.get_comment_lines(), self.get_empty_lines(),
        )
    }
}

impl FolderResult {
    pub fn increment_empty_lines(&mut self) {
        self.empty_lines += 1;
    }

    pub fn increment_code_lines(&mut self) {
        self.code_lines += 1;
    }

    pub fn increment_comment_lines(&mut self) {
        self.comment_lines += 1;
    }

    pub fn decrement_code_lines(&mut self) {
        self.code_lines -= 1;
    }

    pub fn increment_file_number(&mut self) {
        self.file_number += 1;
    }

    pub fn increment_code_file_number(&mut self) {
        self.code_file_number += 1;
    }

    pub fn increment_lines(&mut self, number: i32) {
        self.lines += number;
    }

    pub fn get_empty_lines(&self) -> i32 {
        self.empty_lines
    }

    pub fn get_comment_lines(&self) -> i32 {
        self.comment_lines
    }

    pub fn get_code_lines(&self) -> i32 {
        self.code_lines
    }

    pub fn get_code_file_number(&self) -> i32 {
        self.code_file_number
    }

    pub fn get_lines(&self) -> i32 {
        self.lines
    }

    pub fn get_file_number(&self) -> i32 {
        self.file_number
    }
}
