mod compute_submission;
mod get_args;
mod get_orig_state;
mod orig_state;
mod submission;
mod utilities;
mod write_submission;

use compute_submission::compute_submission;
use get_args::get_args;
use get_orig_state::get_orig_state;
use orig_state::OrigState;
use write_submission::write_submission;

fn sort_libs(orig_state: &mut OrigState) {
    orig_state
        .libs
        .sort_by(|l1, l2| l1.books.len().cmp(&l2.books.len()));
}

fn main() -> Result<(), String> {
    let args = get_args()?;
    let mut orig_state = get_orig_state(&args.file_name)?;

    sort_libs(&mut orig_state);

    let submission = compute_submission(&orig_state);

    write_submission(&args.file_name, &submission);

    Ok(())
}
