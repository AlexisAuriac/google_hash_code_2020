#[derive(Debug)]
pub struct LibraryScan {
    pub id: usize,
    pub books: Vec<usize>,
}

#[derive(Debug)]
pub struct Submission {
    pub libs: Vec<LibraryScan>,
}
