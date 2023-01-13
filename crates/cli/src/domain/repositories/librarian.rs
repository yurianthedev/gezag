pub trait Librarian: super::Resources {}

impl<T> Librarian for T where T: super::Resources {}
