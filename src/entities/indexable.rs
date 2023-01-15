pub trait Indexable<T> {
    fn is_related_to(&self, index: &T) -> bool;
}

