use crate::util::actor::Msg;
pub mod get_by_key;

#[derive(Debug)]
pub enum WanQuery {
    GetWanById(Msg<get_by_key::GetWanById>),
    GetWanByName(Msg<get_by_key::GetWanByName>)
}




// // GetByNetName
// pub struct GetWanByName<T> where T: View {
//     pub name: NetName,
//     pub view: PhantomData<T>
// }

// impl<T> DbQuery for GetWanByName<T> where T: View {
//     type Ok = Option<T>;

//     fn run(self, db: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
//         let query = GetByKey {
//             key: "name",
//             value: self.name,
//             view: PhantomData::<T>
//         };
//         query.run(&db)
//     }
// }

// pub struct GetWanById<T> where T: View {
//     pub id: WanId,
//     pub view: PhantomData<T>
// }

// impl<T> DbQuery for GetWanById<T> where T: View {
//     type Ok = Option<T>;

//     fn run(self, db: &Transaction) -> Result<Self::Ok, rusqlite::Error> {
//         let query = GetByKey {
//             key: "id",
//             value: self.id,
//             view: PhantomData::<T>
//         };
//         query.run(&db)
//     }
// }
