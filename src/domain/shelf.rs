// use super::{
//     error::GenericError,
//     resource::{Obtain, Resource},
// };

// /// Shelf is an abstraction for a collection of resources.
// /// Shelf can be distributed.
// /// They can be a simple local shelf, or a fully fledge cloud-based shelf, is up to the implementer, but they must be able to compose.
// pub trait Shelf {
//     type Key;
//     type Resource: Resource<Key = Self::Key>;
//     type Obtain: Obtain<Resource = Self::Resource>;

//     fn is_resource_fetched(&self, key: Self::Key) -> bool;

//     fn try_fetch(&self, obtainable: Self::Obtain) -> Result<url::Url, GenericError>;

//     fn try_unfetch(&self, obtainable: Self::Obtain) -> Result<(), GenericError>;
// }
