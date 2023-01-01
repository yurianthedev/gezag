use std::hash;

// pub trait Key {
//     type Key: PartialEq + PartialOrd + hash::Hash + Eq + Ord;

//     fn key(&self) -> &Self::Key;
// }

// pub trait Obtain {
//     type Resource: Resource;

//     fn obtain(&self) -> url::Url;
// }

// pub trait Resource: Key {}

pub struct Book {
    key: uuid::Uuid,
}
