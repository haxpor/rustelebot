use crate::ErrorResult;

use std::fmt::{Display, Formatter, Error};

impl Display for ErrorResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.msg)
    }
}
