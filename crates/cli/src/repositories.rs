pub mod repository;

use crate::entities::resources::{Keys, Resource, Indeces};

use self::repository::{Add, List, ListBy, Remove};

pub trait ResourceRepository:
    Add<Resource> + Remove<Resource, K = Keys> + List<Resource> + ListBy<Resource, I = Indeces>
{
}
