mod compute_submission;
mod get_args;
mod get_orig_state;
mod utilities;

use compute_submission::compute_submission;
use get_args::get_args;
use get_orig_state::get_orig_state;

// fn write_submission(example_file_name: &str, submission: Submission) {}

fn main() -> Result<(), String> {
    let args = get_args()?;
    let orig_state = get_orig_state(&args.file_name)?;

    let submission = compute_submission(&orig_state);

    println!("{:?}", submission);

    Ok(())
}
