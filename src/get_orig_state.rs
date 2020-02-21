use crate::orig_state::{LibraryDesc, OrigState};
use crate::utilities::{load_file, parse_pretty_err};

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

fn parse_book_line(book_line: &str, books: &mut Vec<usize>) -> Result<(), String> {
    let line_parts = book_line.split_whitespace().collect::<Vec<_>>();

    for s in line_parts {
        books.push(parse_pretty_err(s)?);
    }

    Ok(())
}

fn zip_libs_with_books(
    nb_libs: usize,
    file_lines: &[&str],
) -> Result<Vec<(String, String)>, String> {
    if file_lines.len() % 2 != 0 {
        return Err("Not all libs have a book list".to_string());
    } else if file_lines.len() / 2 != nb_libs {
        return Err("There are too many libraries".to_string());
    }

    let mut libs_and_books = Vec::with_capacity(nb_libs);

    for i in (0..nb_libs * 2).step_by(2) {
        libs_and_books.push((file_lines[i].to_string(), file_lines[i + 1].to_string()));
    }

    Ok(libs_and_books)
}

fn parse_lib_desc(lib_desc: &str) -> Result<(usize, usize, usize), String> {
    let line_parts = lib_desc.split_whitespace().collect::<Vec<_>>();

    if line_parts.len() != 3 {
        return Err("Invalid lib description".to_string());
    }

    Ok((
        parse_pretty_err(line_parts[0])?,
        parse_pretty_err(line_parts[1])?,
        parse_pretty_err(line_parts[2])?,
    ))
}

fn parse_lib(id: usize, lib_desc: &str, books: &str) -> Result<LibraryDesc, String> {
    let (nb_books, signup_time, ship_per_day) = parse_lib_desc(lib_desc)?;
    let book_ids = books
        .split_whitespace()
        .map(parse_pretty_err)
        .collect::<Result<Vec<usize>, _>>()?;

    if nb_books < book_ids.len() {
        return Err("There are more books than was said in the description".to_string());
    }

    Ok(LibraryDesc {
        id,
        signup_time,
        books: book_ids,
        ship_per_day,
    })
}

fn parse_file_lines(file_lines: &mut Vec<&str>) -> Result<OrigState, String> {
    if file_lines.len() < 2 {
        return Err("Not enough lines to parse".to_string());
    }

    let (nb_books, nb_libs, nb_days) = parse_first_line(file_lines.remove(0))?;
    let mut orig_state = OrigState::new(nb_books, nb_libs, nb_days);

    parse_book_line(file_lines.remove(0), &mut orig_state.books)?;

    let libs_and_books = zip_libs_with_books(nb_libs, file_lines)?;

    for (i, (lib_desc, books)) in libs_and_books.iter().enumerate() {
        orig_state.libs.push(parse_lib(i, &lib_desc, &books)?);
    }

    Ok(orig_state)
}

pub fn get_orig_state(file_name: &str) -> Result<OrigState, String> {
    let file_content = load_file(file_name)?;
    let mut lines = file_content
        .split('\n')
        .filter(|s| s != &"")
        .collect::<Vec<_>>();

    match lines.last() {
        Some(_) => parse_file_lines(&mut lines),
        _ => Err("Empty file".to_string()),
    }
}
