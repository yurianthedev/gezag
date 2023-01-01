use super::error::GenericError;

pub trait Add<Item> {
    /// Self and how it stores resources is unimportant, that's why it takes a ref instead of a mut ref.
    fn add(&mut self, item: Item) -> Result<(), GenericError>;
}

pub trait List<Item> {
    fn list(&self) -> Result<Vec<Item>, GenericError>;
}

pub trait Remove<Key> {
    fn remove(&self, key: &Key) -> Result<(), GenericError>;
}

pub trait Update<Key, Item> {
    fn update(&self, key: Key, item: Item) -> Result<(), GenericError>;
}

pub trait Repository<Key, Item>: Add<Item> + List<Item> + Remove<Key> + Update<Key, Item> {}
