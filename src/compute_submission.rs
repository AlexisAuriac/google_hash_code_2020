use crate::orig_state::{LibraryDesc, OrigState};
use crate::submission::{LibraryScan, Submission};

fn get_book_shipment(
    scanned_books: &[usize],
    nb_to_ship: usize,
    total_books: &mut Vec<usize>,
) -> Vec<usize> {
    let mut shipped = 0;
    let mut to_ship = vec![];

    while !total_books.is_empty() {
        let book = total_books.pop().unwrap();

        if shipped == nb_to_ship {
            break;
        }

        if !scanned_books.contains(&book) {
            to_ship.push(book);
            shipped += 1;
        }
    }

    to_ship
}

fn get_scannable_books(
    orig_state: &OrigState,
    day: usize,
    scanned_books: &[usize],
    lib: &LibraryDesc,
) -> Vec<usize> {
    let mut scannable_books = vec![];
    let mut total_books = lib.books.clone();

    for _ in day..orig_state.nb_days {
        if total_books.is_empty() {
            break;
        }

        let book_shipment = get_book_shipment(&scanned_books, lib.ship_per_day, &mut total_books);

        scannable_books.extend_from_slice(&book_shipment[..]);
    }

    scannable_books
}

pub fn compute_submission(orig_state: &OrigState) -> Submission {
    let mut day = 0;
    let mut libs = vec![];
    let mut scanned_books = vec![];

    for lib in &orig_state.libs {
        if day + lib.signup_time >= orig_state.nb_days {
            continue;
        }

        day += lib.signup_time;
        let scannable_books = get_scannable_books(orig_state, day, &scanned_books, lib);
        scanned_books.extend_from_slice(&scannable_books[..]);

        libs.push(LibraryScan {
            id: lib.id,
            books: scannable_books,
        });
    }

    Submission { libs }
}
