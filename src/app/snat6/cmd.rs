use serde::{Deserialize, Serialize};
use crate::{app::{actor::AppActor, error::AppError}, util::{actor::{Payload, Process}, domain::Id}};
use super::{models::*, query::*};

/// Command that creates a Source NAT Entry
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSNat6 {
    pub prefix: SNat6Prefix
}

impl Payload for CreateSNat6 {
    type Ok = Id;
    type Err = AppError;
}

impl Process for CreateSNat6 {
    type Actor = AppActor;
    
    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let query = ComputeSNat6Prefix { prefix: self.prefix };
        let iprefix = db.run(query)?;

        let query = GetSNat6Overlappings { prefix: iprefix };
        let overlaps = db.run(query)?;
        if !overlaps.is_empty() {
            Err(AppError::SNat6PrefixOverlaps { prefix: self.prefix, iprefix, overlaps })?;
        }

        let mut nat = SNat6::new(Id::new(), (self.prefix, iprefix));
        db.save(&mut nat)?;
        Ok(nat.id)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AddSNat6Target {
    pub id: Id,
    pub prefix: SNat6TargetPrefix
}

impl Payload for AddSNat6Target {
    type Ok = ();
    type Err = AppError;
}

impl Process for AddSNat6Target {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut snat = db.run(GetSNat6ById { id: self.id })?;

        let query = ComputeSNat6TargetPrefix { prefix: self.prefix };
        let iprefix = db.run(query)?;

        let query = GetSNat6TargetOverlappings { prefix: iprefix };
        let overlaps = db.run(query)?;
        if !overlaps.is_empty() {
            Err(AppError::SNat6TargetPrefixOverlaps { prefix: self.prefix, iprefix, overlaps })?;
        }

        snat.add_target(SNat6Target::new(Id::new(), snat.id, self.prefix, iprefix));
        db.save(&mut snat)?;
        Ok(())
    }
}

pub struct UpdateSNatTarget {
    pub id: Id,
    pub snat_id: Id,
    pub prefix: SNat6TargetPrefix
}

impl Payload for UpdateSNatTarget {
    type Ok = ();
    type Err = AppError;
}

impl Process for UpdateSNatTarget {
    type Actor = AppActor;

    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut snat = db.run(GetSNat6ById { id: self.snat_id })?;
        let mut target = db.run(GetSNatTargetById { id: self.id, snat_id: self.snat_id })?;
        
        let query = ComputeSNat6TargetPrefix { prefix: self.prefix };
        let iprefix = db.run(query)?;

        let query = GetSNat6TargetOverlappings { prefix: iprefix };
        let overlaps = db.run(query)?;
        if !overlaps.is_empty() {
            Err(AppError::SNat6TargetPrefixOverlaps { prefix: self.prefix, iprefix, overlaps })?;
        }

        target.prefix = self.prefix;
        snat.update_target(target);
        db.save(&mut snat)?;
        Ok(())
    }
}

pub struct SetSNat6Mode {
    pub id: Id,
    pub mode: SNat6Mode
} 

impl Payload for SetSNat6Mode {
    type Ok = ();
    type Err = AppError;
}

impl Process for SetSNat6Mode {
    type Actor = AppActor;
    
    fn process(self, actor: &mut AppActor) -> Result<Self::Ok, Self::Err> {
        let db = actor.db.lock().begin()?;
        let mut snat = db.run(GetSNat6ById { id: self.id })?;
    
        if self.mode == snat.mode {
            Err(AppError::SNat6ModeAlreadyInUse { mode: self.mode.clone() })?;
        }
        if let Some(unknowns) = snat.targets.unknowns(snat.mode.targets()) {
            Err(AppError::SNat6ModeHasUnknownTargets { mode: self.mode.clone(), unknowns })?;
        }

        snat.set_mode(self.mode);        
        if snat.status == SNat6Status::Enabled {
            let query = ComputeSNatStatusForMode { mode: snat.mode.clone() };
            let status = db.run(query)?;
            snat.set_status(status);
        }
        db.save(&mut snat)?;
        Ok(())
    }
}


// Rules need to be rebuild when
// The status of an ISP Wan Link changes
// The prefix of a Lan or Wan changes 
pub struct RebuildRules {

}

// pub struct EnableSNatFailoverEgressCmd {
//     pub id: Id,
//     pub mapping_ids: BTreeSet<Id>
// }

// impl Request for EnableSNatFailoverEgressCmd {
//     type Ok = ();
//     type Err = AppError;
//     type Processor = AppActor;

//     fn process(self, actor: &mut AppActor) -> Result<(), AppError> {
//         let db = actor.db.lock().retry(2, Duration::from_millis(500)).begin()?;
//         let mut nat = match db.run(&GetSNat6ById { id: self.id })? {
//             Some(nat) => nat,
//             None => Err(AppError::NotFound { id: self.id })?
//         };

//         // // TBD: Check if Another SNAT Entry is already enabled 
//         // if let Some(foreign_mappings) = nat.get_foreign_mappings(&mapping_ids) {
//         //     log.errors += AppError::CantEnableEgressAsForeignMapppingsWereFound { id: nat.id, foreign_mappings };
//         //     return log.err();
//         // }
//         // if let Some(overlaps) = self.get_target_overlaps(&mapping_ids).await {
//         //     log.errors += AppError::CantEnableEgressAsOverlappingMappingsWereFound { id: nat.id, overlaps };
//         //     return log.err();
//         // }
//         // if mapping_ids.len() < 2 {
//         //     log.errors += AppError::CantEnableEgressAsItRequiresMoreMappings { id: nat.id, min_expected: 2, found: mapping_ids.len() };
//         //     return log.err();
//         // }

//         let egress = SNatEgress::Failover(BTreeSet::from_iter(self.mapping_ids.into_iter()));
//         nat.set_egress(egress);
//         let _ = db.save(nat);
//         Ok(())
//     }
// }

// pub struct EnableSNatLoadsharedEgressCmd {
//     pub id: Id,
//     pub mapping_ids: HashSet<Id>
// }

// impl Request for EnableSNatLoadsharedEgressCmd {
//     type Ok = ();
//     type Err = AppError;
//     type Processor = AppActor;

//     fn process(self, actor: &mut AppActor) -> Result<(), AppError> {
//         let db = actor.db.lock().retry(2, Duration::from_millis(500)).begin()?;
//         let mut nat = match db.run(&GetSNat6ById { id: self.id })? {
//             Some(nat) => nat,
//             None => Err(AppError::NotFound { id: self.id })?
//         };
           
//         // if let Some(foreign_mappings) = nat.get_foreign_mappings(&mapping_ids) {
//         //     log.errors += AppError::CantEnableEgressAsForeignMapppingsWereFound { id: nat.id, foreign_mappings };
//         //     return log.err();
//         // }
//         // if let Some(overlaps) = self.get_target_overlaps(&mapping_ids).await {
//         //     log.errors += AppError::CantEnableEgressAsOverlappingMappingsWereFound { id: nat.id, overlaps };
//         //     return log.err();
//         // }
//         // if mapping_ids.len() < 2 {
//         //     log.errors += AppError::CantEnableEgressAsItRequiresMoreMappings { id: nat.id, min_expected: 2, found: mapping_ids.len() };
//         //     return log.err();
//         // }

//         let egress = SNatEgress::Loadshare(HashSet::from_iter(self.mapping_ids.into_iter()));
//         nat.set_egress(egress);
//         let _ = db.save(nat);
//         Ok(())
//     }
// }

// pub struct DisableSNatEgressCmd {
//     pub id: Id
// }

// impl Request for DisableSNatEgressCmd {
//     type Ok = ();
//     type Err = AppError;
//     type Processor = AppActor;

//     fn process(self, actor: &mut AppActor) -> Result<(), AppError> {
//         Ok(())
//         // let mut log = SNatIssueLogger::new();
//         // match Id::from_str(&cmd.id) {
//         //     Ok(id) => {
//         //         let mut nat: SNat = match db::snat::get_by_id(id).await {
//         //             Some(nat) => nat,
//         //             None => {
//         //                 log.errors += AppError::CantUpdateSNatAsItWasNotFound { id };
//         //                 return log.err();
//         //             }
//         //         };
                
//         //         nat.disable_egress();
//         //         let _ = db::snat::save(nat).await;
//         //         log.ok(())
//         //     },
//         //     Err(error) => {
//         //         log.errors += AppError::BadId { value: cmd.id };
//         //         return log.err();
//         //     }
//         // }
//     }
// }



// pub async fn get_target_overlaps_for(network: &WanNetwork) -> Option<TargetOverlaps> {
//     None
//     // AppError::CantMapToTargetAsItWouldOverlap
//     // OverlappinGroup { network: , network_prefix: , Vec<> }
//     // Overlapping Groups HashMap<Ipv6Net Per Mapping, 
// }

// pub async fn get_target_overlaps(mapping_ids: &Vec<Id>) -> Option<Vec<TargetOverlaps>> {
//     None
//     // use get_taget_overlap for each mapping network
// }

// async fn build_snat_policy(&self, policy: SNatPolicyName, ids: Vec<Id>) -> Result<SNatPolicy, SNatIssueLogger> {
    //     let mut log = SNatIssueLogger::new();
    //     match policy {
    //         SNatPolicyName::Single =>  {
    //             match self.check_mapping_ids(ids,  |len| len == 1, AppError::PolicyMustOnlyContainOneIsp) {
    //                 Ok(ids) => return Ok(SNatPolicy::Single(ids.into_iter().next().unwrap())),
    //                 Err(errors) => {
    //                     log.errors += errors;
    //                     return Err(e);
    //                 }
    //             }
    //         },
    //         SNatPolicyName::Failover => {
    //             match self.check_mapping_ids(ids, |len| len >= 2, AppError::PolicyRequiresTwoOrMoreDifferentIsps) {
    //                 Ok(ids) => return Ok(SNatPolicy::Failover(BTreeSet::from_iter(ids.into_iter()))),
    //                 Err(errors) => {
    //                     log.errors += errors;
    //                     return Err(e)
    //                 }
    //             }
    //         },
    //         SNatPolicyName::Loadshare => {
    //             match self.check_mapping_ids(ids, |len| len >= 2, AppError::PolicyRequiresTwoOrMoreDifferentIsps) {
    //                 Ok(ids) => return Ok(SNatPolicy::Loadshare(ids)),
    //                 Err(errors) => {
    //                     log.errors += errors;
    //                     return Err(e)
    //                 }
    //             }
    //         }
    //     }
    // }

    // async fn check_mapping_ids(&self, ids: Vec<SNatTarget>, count_predicate: impl Fn(usize) -> bool, count_error: AppError) -> Result<HashSet<WanId>, SNatIssueLogger> {
    //     let mut log = SNatIssueLogger::new();
    //     let len = ids.len();
    //     let unique_ids = HashSet::from_iter(ids.into_iter()); // deduping
    //     if len != unique_ids.len() {
    //         log.errors += AppError::PolicyIncludesDuplicatedMappings;
    //         return Err(e);
    //     } 

    //     if !count_predicate(len) {
    //         log.errors += count_error;
    //     }
    //     for id in unique_ids.iter() {
    //         if get_network_by_id(&id).await.is_none() {
    //             log.errors += AppError::MappingInPolicyNotFound { id: *id };
    //         }
    //     } 
    //     if e.is_err() {
    //         return Err(e);
    //     } 
    //     Ok(unique_ids)
    // }  