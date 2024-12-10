use std::marker::PhantomData;
use crate::app::data::DbSession;
use crate::{app::{actor::AppActor, data::framework::traits::DbQuery, error::AppError, shared::query::*}, sys::link::models::LinkId, util::{actor::{Payload, Process}, domain::Id}};
use super::models::*;

pub struct GetAllIsp;

impl Payload for GetAllIsp {
    type Err = AppError;
    type Ok = Vec<Isp>;
}

impl Process for GetAllIsp {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetAllIsp {
    type Err = AppError;
    type Ok = Vec<Isp>;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetAll { entity: PhantomData::<Isp> };
        db.run(query)
    }
}

/// Query that returns the ISP with the specified id
pub struct GetIspById {
    pub id: Id
}

impl Payload for GetIspById {
    type Err = AppError;
    type Ok = Isp;
}

impl Process for GetIspById {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetIspById {
    type Ok = Isp;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Isp>,
            field_name: "id",
            field_value: self.id
        };
        query.run(db)
    }
}

/// Query that returns the ISP with the specified name
pub struct GetIspByName {
    pub name: IspName
}

impl Payload for GetIspByName {
    type Err = AppError;
    type Ok = Isp;
}

impl Process for GetIspByName {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetIspByName {
    type Ok = Isp;
    type Err = AppError;

    fn run(self, db: &crate::app::data::DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            field_name: "name",
            field_value: self.name,
            entity: PhantomData::<Isp>
        };
        query.run(&db)
    }
}

/// Query that returns the ISP using the specified link
pub struct GetIspByLink {
    pub link: LinkId
}

impl Payload for GetIspByLink {
    type Ok = Isp;
    type Err = AppError;
}

impl Process for GetIspByLink {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        self.run(&db)
    }
}

impl DbQuery for GetIspByLink {
    type Ok = Isp;
    type Err = AppError;
    
    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetByField {
            entity: PhantomData::<Isp>,
            field_name: "link",
            field_value: self.link
        };
        query.run(db)
    }
}

/// Query that returns the total number of ISPs
pub struct GetIspCount;

impl Payload for GetIspCount {
    type Ok = usize;
    type Err = AppError;
}

impl Process for GetIspCount {
    type Actor = AppActor;

    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.begin()?;
        db.run(self)
    }
}

impl DbQuery for GetIspCount {
    type Ok = usize;
    type Err = AppError;

    fn run(self, db: &DbSession) -> Result<Self::Ok, Self::Err> {
        let query = GetCount { entity: PhantomData::<Isp> };
        query.run(db)
    }
}


// pub struct GetIspForWan {
//     pub id: Id // wan_id
// }

// // DbQuery table containing (IspId, WanId) pairs and retrieve the IspId listed for wan_id
// impl DbQuery for GetIspForWan {
//     type Result = Isp;

//     fn run(&self, db: &DbSession) -> Result<Self::Result, AppError> {
//         let sql = format!("{} WHERE id = :id AND status = :status", Lan::select());
//         match db.tx().query_row(&sql, named_params! { ":id": self.id, ":status": LanStatus::Nominal }, Lan::map) {
//             Ok(lan) => Ok(lan),
//             Err(Error::DbQueryReturnedNoRows) => Err(LanError::NotFound { id: self.id })?,
//             Err(e) => Err(e)?
//         }
//     }
// }
