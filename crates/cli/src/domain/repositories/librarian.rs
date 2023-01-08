pub trait Librarian: super::ResourcesRepository {}

impl<T> Librarian for T where T: super::ResourcesRepository {}
