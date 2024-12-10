use std::marker::PhantomData;
use crate::{app::{data::{framework::traits::{DbQuery, MapRow}, DbSession}, error::AppError, shared::query::GetAll}, util::net::types::{IpPrefix, IpPrefixOverlap}};
use super::models::Lan;

pub struct GetLanOverlappings<T> where T: Lan {
    pub prefix: T::Prefix,
    pub entity: PhantomData<T>
}

impl<T> DbQuery for GetLanOverlappings<T> where T: Lan + MapRow {
    type Ok = Vec<T>;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll { entity: PhantomData::<T> };
        let lans =  query.run(db)?;
        let overlaps = lans.into_iter().filter(|lan| {
            if let Some(overlap) = self.prefix.overlaps(lan.prefix()) {
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