use std::fs::File;
use std::io::prelude::*;

use crate::submission::{LibraryScan, Submission};

fn lib_scan_to_string(lib_scan: &LibraryScan) -> String {
    let desc = format!("{} {}", lib_scan.id, lib_scan.books.len());
    let books_str = lib_scan
        .books
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    format!("{}\n{}", desc, books_str)
}

fn submission_to_string(submission: &Submission) -> String {
    let first_line = format!("{}", submission.libs.len());
    let lib_lines = submission
        .libs
        .iter()
        .map(|lib| lib_scan_to_string(lib))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}\n", first_line, lib_lines)
}

pub fn write_submission(example_file_name: &str, submission: &Submission) {
    let output_file_name = format!("{}.output", example_file_name);
    let mut file = File::create(output_file_name).unwrap();

    let string_submission = submission_to_string(submission);

    file.write_all(string_submission.as_bytes()).unwrap();
}
