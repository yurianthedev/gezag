pub mod resources;
pub mod topics;

pub trait Key<T> {
    fn is(&self, el: &T) -> bool;
}

pub trait Index<T> {
    fn is_related(&self, el: &T) -> bool;
}
