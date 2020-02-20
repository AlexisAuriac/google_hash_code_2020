use std::fs::File;
use std::io::prelude::*;

mod compute_submission;
mod get_args;
mod get_orig_state;
mod utilities;

use compute_submission::{compute_submission, LibraryScan, Submission};
use get_args::get_args;
use get_orig_state::{get_orig_state, OrigState};

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

fn write_submission(example_file_name: &str, submission: &Submission) {
    let output_file_name = format!("{}.output", example_file_name);
    let mut file = File::create(output_file_name).unwrap();

    let string_submission = submission_to_string(submission);

    file.write_all(string_submission.as_bytes()).unwrap();
}

fn sort_libs(orig_state: &mut OrigState) {
    orig_state
        .libs
        .sort_by(|l1, l2| l1.signup_time.cmp(&l2.signup_time));
}

fn main() -> Result<(), String> {
    let args = get_args()?;
    let mut orig_state = get_orig_state(&args.file_name)?;

    sort_libs(&mut orig_state);

    let submission = compute_submission(&orig_state);

    write_submission(&args.file_name, &submission);

    Ok(())
}
