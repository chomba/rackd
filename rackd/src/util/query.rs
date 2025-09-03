use std::fmt::Display;
use thiserror::Error;
use crate::util::api;

#[derive(Debug, Error)]
pub enum GetByKeyError<K> where K: Display {
    #[error("Database Error: {}", .0)]
    Db(#[from] rusqlite::Error),
    #[error("{} not found", .0)]
    NotFound(K),
    // TBD: Change NetlinkError
    #[error("{}", .0)]
    Netlink(String)
}

impl<K> From<GetByKeyError<K>> for api::Error where K: Display {
    fn from(error: GetByKeyError<K>) -> Self {
        let msg = error.to_string();
        match error {
            GetByKeyError::Db(_) => api::Error::new("QUERY_DB_ERROR", msg),
            GetByKeyError::Netlink(_) => api::Error::new("QUERY_NETLINK_ERROR", msg),
            GetByKeyError::NotFound(key) => api::Error::new("QUERY_ENTITY_NOT_FOUND", msg)
        }
    }
} 