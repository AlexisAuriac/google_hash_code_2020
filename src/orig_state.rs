#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LibraryDesc {
    pub id: usize,
    pub signup_time: usize,
    pub books: Vec<usize>,
    pub ship_per_day: usize,
}

#[derive(Debug)]
pub struct OrigState {
    pub nb_books: usize,
    pub libs: Vec<LibraryDesc>,
    pub nb_days: usize,
    pub books: Vec<usize>,
}

impl OrigState {
    pub fn new(nb_books: usize, nb_libs: usize, nb_days: usize) -> OrigState {
        OrigState {
            nb_books,
            libs: Vec::with_capacity(nb_libs),
            nb_days,
            books: Vec::with_capacity(nb_books),
        }
    }
}
