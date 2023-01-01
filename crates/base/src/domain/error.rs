use std::error;

pub type GenericError = Box<dyn error::Error>;
