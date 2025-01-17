use std::any::type_name;

use rusqlite::Transaction;

use super::models::Event;

pub trait OptionExt<T> {
    fn err_or<E>(self, err: E) -> Result<(), E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn err_or<E>(self, err: E) -> Result<(), E> {
        match self {
            Some(_) => Err(err),
            None => Ok(())
        }
    }
}