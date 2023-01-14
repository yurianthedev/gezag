pub trait Librarian: super::Resources + super::Topics {}

impl<T> Librarian for T where T: super::Resources + super::Topics {}
