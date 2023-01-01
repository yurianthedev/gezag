use super::error::GenericError;

pub trait Add {
    type Item;

    /// Self and how it stores resources is unimportant, that's why it takes a ref instead of a mut ref.
    fn add(&self, item: Self::Item) -> Result<(), GenericError>;
}

pub trait List {
    type Item;

    fn list(&self) -> Result<Vec<Self::Item>, GenericError>;
}

pub trait Remove {
    type Item;

    fn remove<Key: PartialEq + PartialOrd>(&self, key: &Key) -> Result<(), GenericError>;
}

pub trait Update {
    type Item: PartialEq + PartialOrd; // TODO: I'd like to explore the idea of having two separate `Update` traits. One for updating just with the item, in case this is comparable by some sort of key. The other for updating the item by handling explicitly a key, in case Item does not have one – or is not longer reliable.

    fn update(&self, item: Self::Item) -> Result<(), GenericError>;
}

pub trait Repository:
    Add<Item = Self> + List<Item = Self> + Remove<Item = Self> + Update<Item = Self>
where
    Self: std::marker::Sized,
{
}
