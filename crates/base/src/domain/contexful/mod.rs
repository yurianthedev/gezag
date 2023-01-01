pub mod check_item;

use super::error::GenericError;
use check_item::CheckItem;

pub trait Contexful {
    /// Run for the first time. Read relevant configuration and run validations if needed.
    fn init(&self) -> Result<(), GenericError>;

    /// For example, the settings for the shelf have been update externally (e.g. by updating a config file manually).
    fn update(&mut self) -> Result<(), GenericError>;

    /// Is the shelf working as expected? Get back a diagnose.
    fn doctor(&self) -> Vec<CheckItem>;
}
