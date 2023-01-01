#[derive(Clone)]
pub enum Status {
    Ok,
    Faulty { reason: String },
}

impl From<Status> for bool {
    fn from(value: Status) -> Self {
        match value {
            Status::Ok => true,
            Status::Faulty { reason: _ } => false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Required(pub bool);

#[derive(Clone)]
pub struct CheckItem {
    required: Required,
    description: String,
    status: Status,
}

/// Let's say you have an Iterator<Item = CheckItem>. Well, you might be interested in knowing if there's a problem/warning/error with any of them.
trait CheckItemsIterExt: Iterator<Item = CheckItem>
where
    Self: std::marker::Sized,
{
    fn any_problems(&mut self) -> bool {
        self.any(|ci| ci.status.into())
    }

    fn any_warnings(&mut self) -> bool {
        self.any(|ci| !ci.required.0 && !bool::from(ci.status))
    }

    fn any_errors(&mut self) -> bool {
        self.any(|ci| ci.required.0 && !bool::from(ci.status))
    }
}
