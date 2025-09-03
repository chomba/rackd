use std::{any::type_name, marker::PhantomData};
use log::error;
use rusqlite::{named_params, Transaction};
use crate::{db::query::traits::{DbQuery, GetByKey, QueryRunner, DbView}, trunk::model::TrunkId};
use super::{model::values::{NetName, VlanId}, views::NetworkView};

pub struct GetByName<'a, T> where T: DbView {
    pub name: &'a NetName,
    pub view: PhantomData<T>
}

impl<'a, T> DbQuery for GetByName<'a, T> where T: DbView {
    type Ok = Option<T>;

    fn run(&self, db: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let query = GetByKey {
            key: "name",
            value: self.name,
            view: PhantomData::<T>
        };
        query.run(&db)
    }
}

pub struct GetByTrunkVlan<T> where T: DbView {
    pub trunk: TrunkId,
    pub vlan: VlanId,
    pub view: PhantomData<T>
}

impl<T> DbQuery for GetByTrunkVlan<T> where T: DbView {
    type Ok = Option<T>;
    
    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        let sql = format!("{} WHERE vlan = :vlan AND trunk_id = :trunk_id AND deleted = :deleted", T::sql_select());
        match tx.query_row(&sql, named_params! { ":vlan": self.vlan, ":trunk_id": self.trunk, ":deleted": false }, T::try_from) {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).map_err(|e| { 
                error!("query_row() in GetByKey<{}> failed: {}", type_name::<T>(), e); 
                e 
            })?
        }
    }
}

pub struct GetNetworkByName {
    pub name: NetName
}

impl DbQuery for GetNetworkByName {
    type Ok = Option<NetworkView>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        tx.run(GetByName { name: &self.name, view: PhantomData::<NetworkView> })
    }
}

pub struct GetNetworkByTrunkVlan {
    pub trunk: TrunkId,
    pub vlan: VlanId
}

impl DbQuery for GetNetworkByTrunkVlan {
    type Ok = Option<NetworkView>;

    fn run(&self, tx: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
        tx.run(GetByTrunkVlan { trunk: self.trunk, vlan: self.vlan, view: PhantomData::<NetworkView> })
    }
}