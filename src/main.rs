mod get_args;
mod get_orig_state;
mod utilities;

use get_args::get_args;
use get_orig_state::get_orig_state;

fn main() -> Result<(), String> {
    let args = get_args()?;
    let orig_state = get_orig_state(&args.file_name)?;

    println!("original state: {:?}", orig_state);

    Ok(())
}
