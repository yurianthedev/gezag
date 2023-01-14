pub trait Keyable<T> {
    fn is(&self, key: &T) -> bool;
}
