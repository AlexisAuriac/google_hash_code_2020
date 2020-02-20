mod get_args;
mod utilities;

use get_args::get_args;
use utilities::{load_file, parse_pretty_err};

#[derive(Debug)]
struct LibraryDesc {
    signup_time: usize,
    books: Vec<usize>,
    ship_per_day: usize,
}

#[derive(Debug)]
struct OrigState {
    nb_books: usize,
    libraries: Vec<LibraryDesc>,
    nb_days: usize,
}

fn parse_first_line(first_line: &str) -> Result<(usize, usize, usize), String> {
    let line_parts = first_line.split_whitespace().collect::<Vec<_>>();

    if line_parts.len() != 3 {
        return Err("Invalid first line".to_string());
    }

    Ok((
        parse_pretty_err(line_parts[0])?,
        parse_pretty_err(line_parts[1])?,
        parse_pretty_err(line_parts[2])?,
    ))
}

fn parse_file_lines(file_lines: &mut std::vec::Vec<&str>) -> Result<OrigState, String> {
    let (nb_books, nb_libs, nb_days) = parse_first_line(file_lines.remove(0))?;

    println!("nb_books: {:?}", nb_books);
    println!("nb_libs: {:?}", nb_libs);
    println!("nb_days: {:?}", nb_days);

    unimplemented!()
}

fn get_orig_state(file_name: &str) -> Result<OrigState, String> {
    let file_content = load_file(file_name)?;
    let mut lines = file_content.split('\n').collect::<Vec<_>>();

    match lines.last() {
        Some(&"") => {
            lines.pop();
            parse_file_lines(&mut lines)
        }
        Some(_) => parse_file_lines(&mut lines),
        _ => Err("Empty file".to_string()),
    }
}

fn main() -> Result<(), String> {
    let args = get_args()?;
    let orig_state = get_orig_state(&args.file_name)?;

    Ok(())
}
