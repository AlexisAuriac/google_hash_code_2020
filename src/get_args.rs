#[derive(Debug)]
pub struct Args {
    pub file_name: String,
}

pub fn get_args() -> Result<Args, String> {
    let raw_args = std::env::args().collect::<Vec<_>>();

    match &raw_args[..] {
        [_, file] => Ok(Args {
            file_name: file.to_string(),
        }),
        [_] => Err("Missing file argument".to_string()),
        _ => Err("Too much arguments, only need 1 file name".to_string()),
    }
}
