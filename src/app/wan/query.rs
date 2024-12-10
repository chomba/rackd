use std::marker::PhantomData;
use crate::{app::{data::{framework::traits::{DbQuery, MapRow}, DbSession}, error::AppError, shared::query::GetAll}, util::net::types::{IpPrefix, IpPrefixOverlap}};
use super::models::Wan;

pub struct GetWanOverlappings<T> where T: Wan  {
    pub prefix: T::Prefix,
    pub entity: PhantomData<T>
}

impl<T> DbQuery for GetWanOverlappings<T> where T: Wan + MapRow {
    type Ok = Vec<T>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll { entity: PhantomData::<T> };
        let wans = query.run(db)?;
        let overlaps = wans.into_iter().filter(|wan| {
            if let Some(overlap) = self.prefix.overlaps(wan.prefix()) {
                return match overlap {
                    IpPrefixOverlap::<T::Prefix>::Partial(_) | IpPrefixOverlap::Equal => true,
                    _ => false
                }
            }
            false
        }).collect();

        Ok(overlaps)
    }
}
