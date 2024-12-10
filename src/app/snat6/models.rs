use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::app::shared::domain::Entity;
use crate::app::shared::domain::Metadata;
use crate::util::domain::Id;
use crate::util::net::types::{Ipv6Prefix, Ipv6PrefixExt};

// Each Mapping should have an ID
// @lan::2401/64 -> MAP POOL: @isp1::2401/80 | Routing Policy: NatFailover between with 1. isp1, 2. isp2 
//               -> MAP POOL: @isp2::2401/80
// @homelab -> MAP POOL: @isp1::2401/80

#[derive(Debug, Default, Clone)]
pub struct SNat6 {
    pub meta: Metadata,
    pub id: Id,
    pub prefix: SNat6Prefix,
    pub iprefix: Ipv6Prefix,
    pub targets: SNat6Targets,
    pub mode: SNat6Mode,
    pub status: SNat6Status
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SNat6Targets(pub HashMap<Id, SNat6Target>);

impl SNat6Targets {
    pub fn has(&self, id: &Id) -> bool {
        self.0.contains_key(id)
    }

    pub fn unknowns(&self, targets: Vec<Id>) -> Option<Vec<Id>> {
        let mut foreigners = vec![];
        for id in targets {
            if !self.has(&id) {
                foreigners.push(id);
            }
        }
        if foreigners.is_empty() {
            return Some(foreigners);
        }
        None
    }
    
    pub fn get(&self, id: Id) -> Option<&SNat6Target> {
        self.0.get(&id)
    }

    pub fn insert(&mut self, id: Id, target: SNat6Target) -> Option<SNat6Target> {
        self.0.insert(id, target)
    }

    pub fn remove(&mut self, id: &Id) -> Option<SNat6Target> {
        self.0.remove(id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SNat6Prefix {
    Literal(Ipv6Prefix),
    Lan(Id),
    LanExtension(Ipv6PrefixExt)
}

impl Default for SNat6Prefix {
    fn default() -> Self {
        SNat6Prefix::Literal(Ipv6Prefix::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SNat6Target {
    // SNatForm: Masquerade/Cone/Full/1To1
    pub id: Id,
    pub snat_id: Id,
    pub prefix: SNat6TargetPrefix, 
    pub iprefix: Ipv6Prefix,
}

impl SNat6Target {
    pub fn new(id: Id, snat_id: Id, prefix: SNat6TargetPrefix, iprefix: Ipv6Prefix) -> Self {
        Self { id, snat_id, prefix, iprefix }
    } 
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SNat6TargetPrefix {
    Wan(Id),
    WanExtension(Ipv6PrefixExt)
}

// impl SNat6Target {
//     pub fn check_for_duplicates(mappings: Vec<Id>) -> Result<Vec<Id>, SNatLogger> {
//         let mut e = SNatLogger::new();
//         let observed = HashMap::<Id, u8>::new();
//         for mapping_id in mappings {
//             match observed.get(&mapping_id) {
//                 Some(count) => { 
//                     if count == 1 {
//                         e += SNatError::MappingListedMoreThanOnce { mapping_id };
//                     }
//                     observed.insert(mapping_id, count + 1);
//                 },
//                 None => { 
//                     observed.insert(mapping_id, 1);
//                 }
//             }
//         }
//         if e.is_err() {
//             Err(e)
//         }
//         Ok(mappings)
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SNat6Status {
    Disabled,
    Enabled,
    SingleEgress(Id),
    MultiEgress(HashSet<Id>)
}

impl Default for SNat6Status {
    fn default() -> Self {
        SNat6Status::Disabled
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SNat6Mode {
    None,
    Single(Id),
    Failover(BTreeSet<Id>),
    Loadshare(HashSet<Id>)
}

impl SNat6Mode {
    pub fn targets(&self) -> Vec<Id> {
        match self {
            SNat6Mode::None => vec![],
            SNat6Mode::Single(id) => vec![*id],
            SNat6Mode::Failover(ids) => ids.into_iter().map(|id| *id).collect(),
            SNat6Mode::Loadshare(ids) => ids.into_iter().map(|id| *id).collect()
        }
    }
}

impl Default for SNat6Mode {
    fn default() -> Self {
        SNat6Mode::None
    }
}

// impl SNat6Mode {
//     pub fn equals(&self, mode: SNat6Mode) -> bool {
//         match mode {
//             Some(mode) if *self == mode => true,
//             Some(_) | None => false
//         }
//     }
// }

// Loadshared(with N) gets degraded to (Loadshared(N-1) IF N > 2) OR (Single IF N <= 2)
// Failover(with N) gets degraded to (Failover(N-1) IF N > 2) OR (Single IF N <= 2) 

// impl SNatPolicy {
//     pub fn get_isps(&self) -> Vec<Id> {
//         match self {
//             SNatPolicy::Single(isp) => vec![*isp],
//             SNatPolicy::Failover(isps) => isps.into_iter().map(|isp| *isp).collect(),
//             SNatPolicy::Loadshare(isps) => isps.into_iter().map(|isp| *isp).collect()
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum SNatPolicyName {
//     Single, Failover, Loadshare
// }

// struct SNatPolicyNameParseError;

// impl FromStr for SNatPolicyName {
//     type Err = SNatPolicyNameParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "single" => Ok(SNatPolicyName::Single),
//             "failover" => Ok(SNatPolicyName::Failover),
//             "loadshare" => Ok(SNatPolicyName::Loadshare),
//             _ => Err(SNatPolicyNameParseError) 
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SNat6Event {
    Created { id: Id, prefix: SNat6Prefix, iprefix: Ipv6Prefix },
    PrefixUpdated { prefix: SNat6Prefix, iprefix: Ipv6Prefix },
    TargetAdded { id: Id, prefix: SNat6TargetPrefix, iprefix: Ipv6Prefix },
    TargetUpdated { id: Id, prefix: SNat6TargetPrefix, iprefix: Ipv6Prefix },
    TargetRemoved { id: Id },
    ModeUpdated { mode: SNat6Mode },
    StatusUpdated { status: SNat6Status }
}


// #[derive(Debug, Serialize)]
// pub struct SNatPolicyParseError;

// impl FromStr for SNatPolicy {
//     type Err = SNatPolicyParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = s.trim().to_lowercase();
//         let policy = match s.as_str() {
//             "single" => SNatPolicy::SingleHomed,
//             "failover" => SNatPolicy::Failover,
//             "loadshare" => SNatPolicy::LoadSharing,
//             _ => return Err(SNatPolicyParseError)
//         };
//         Ok(policy)
//     }
// }

// impl SNatPolicy {
//     pub async fn from(policy: &str, isp_api: &IspApi) -> Result<Self, SNatLogger> {
//         let mut e = SNatLogger::new();
//         let policy = policy.trim().to_lowercase();
//         let (policy_name, isp_names) = match policy.split_once('@') {
//             Some(policy) => policy,
//             None => (policy.as_str(), "") 
//         };
//         let policy = match policy_name {
//             "passthrough" => SNatPolicy::Passthrough,
//             "single" => SNatPolicy::SingleHomed,
//             "failover" => SNatPolicy::Failover,
//             "loadshare" => SNatPolicy::LoadSharing,
//             _ => {
//                 e += SNatError::BadPolicyName { value: String::from(policy) };
//                 return Err(e);
//             }
//         };

//         if isp_names.is_empty() {
//             return Ok(policy)
//         }
//         e = SNatLogger::new();
//         let isps = (isp_api.get_all().await).unwrap().data;
//         match policy {
//             SNatPolicy::Passthrough => {
//                 e += SNatError::PolicyDoesntAllowIsps;
//                 Err(e)
//             },
//             SNatPolicy::SingleHomed => {
//                 match SNatPolicy::get_isp_id(isp_names, &isps) {
//                     Ok(id) => Ok(SNatPolicy::SingleHomedWith(id)),
//                     Err(e) => Err(e)
//                 }
//             },
//             SNatPolicy::LoadSharing => {
//                 match SNatPolicy::get_isp_ids(isp_names, &isps) {
//                     Ok(ids) => Ok(SNatPolicy::LoadSharingWith(ids)),
//                     Err(e) => Err(e)
//                 }
//             },
//             SNatPolicy::Failover => {
//                 match SNatPolicy::get_isp_ids(isp_names, &isps) {
//                     Ok(ids) => Ok(SNatPolicy::FailoverWith(ids)),
//                     Err(e) => Err(e)
//                 }
//             },
//             _ => panic!("There's a logic error in SNatPolicy parsing code")
//         }
//     }

//     fn get_isp_id(name: &str, isps: &Vec<IspSummaryView>) -> Result<Id, SNatLogger> {
//         let mut e = SNatLogger::new();
//         for isp in isps {
//             if isp.name == name {
//                 return Ok(Id::from_str(&isp.id).unwrap());
//             }
//         }
//         e += SNatError::PolicyIspNotFound { name: String::from(name) };
//         Err(e)
//     }

//     fn get_isp_ids(names: &str, isps: &Vec<IspSummaryView>) -> Result<BTreeSet<Id>, SNatLogger> {
//         let mut e = SNatLogger::new();
//         let names: Vec<&str> = names.split(',').collect();
//         let mut ids = BTreeSet::<Id>::new();
//         for name in names {
//             match SNatPolicy::get_isp_id(name, isps) {
//                 Ok(id) => { ids.insert(id); },
//                 Err(errors) => { e += errors; }
//             }
//         }
//         if ids.len() < 2 {
//             e += SNatError::PolicyRequiresTwoOrMoreDifferentIsps;
//         }
//         if e.is_err() {
//             return Err(e);
//         }
//         Ok(ids)
//     }
// }

// #[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct SNatName(pub String);

// #[derive(Debug, Serialize)]
// pub struct SNatNameParseError;

// impl FromStr for SNatName {
//     type Err = SNatNameParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = String::from(s.to_lowercase());
//         let length = s.len();
//         if length >= 3 && length <= 40 {
//             return Ok(SNatName(String::from(s)));
//         }
//         Err(SNatNameParseError)
//         // let mut rules = Rules::new();
//         // rules += StringRule::BoundedLength { min: 0, max: 40 }; 
//         // rules.eval(s, |s| SNatName(s), |value, violations| Error::SNatName(SNatNameError::Invalid { value, violations }))

//     }
// }

impl Entity for SNat6 {
    type Event = SNat6Event;

    fn apply(&mut self, e: &Self::Event) {
        match e {
            SNat6Event::Created { id, prefix, iprefix} => {
                self.id = *id;
                self.prefix = *prefix;
                self.iprefix = *iprefix;
            },
            SNat6Event::PrefixUpdated { prefix, iprefix } => {
                self.iprefix = *iprefix;
                self.prefix = *prefix;
            },
            SNat6Event::ModeUpdated { mode } => {
                self.mode = mode.clone();
            },
            SNat6Event::TargetAdded { id, prefix, iprefix } => {
                let target = SNat6Target {
                    id: *id,
                    snat_id: self.id,
                    prefix: *prefix,
                    iprefix: *iprefix
                };
                self.targets.insert(target.id, target);
            },
            SNat6Event::TargetRemoved { id } => {
                self.targets.remove(id);
            },
            SNat6Event::TargetUpdated { id, prefix, iprefix } => {
                let target = SNat6Target {
                    id: *id,
                    snat_id: self.id,
                    prefix: *prefix,
                    iprefix: *iprefix
                };
                self.targets.insert(target.id, target);
            },
            SNat6Event::StatusUpdated { status } => {
                self.status = status.clone();
            }
        }
    }

    fn id(&self) -> Id { self.id }
    fn metadata(&mut self) -> &mut Metadata { &mut self.meta }
}

impl SNat6 {
    pub fn new(id: Id, prefix: (SNat6Prefix, Ipv6Prefix)) -> Self {
        let e = SNat6Event::Created { id, prefix: prefix.0, iprefix: prefix.1 };
        let mut nat = SNat6::default();
        nat.process(e);
        nat
    }

    pub fn add_target(&mut self, target: SNat6Target) {
        let e = SNat6Event::TargetAdded { id: target.id, prefix: target.prefix, iprefix: target.iprefix };
        self.process(e);
    }

    pub fn update_target(&mut self, target: SNat6Target) {
        if self.targets.has(&target.id) {
            let e = SNat6Event::TargetUpdated { id: target.id, prefix: target.prefix, iprefix: target.iprefix };
            self.process(e);
        }
    }

    pub fn remove_target(&mut self, id: Id) {
        if self.targets.has(&id) {
            let e = SNat6Event::TargetRemoved { id };
            self.process(e);
        }
    }

    pub fn set_mode(&mut self, mode: SNat6Mode) {
        let e = SNat6Event::ModeUpdated { mode };
        self.process(e);
    }

    pub fn set_status(&mut self, status: SNat6Status) {
        let e = SNat6Event::StatusUpdated { status };
        self.process(e);
    }

    pub fn enable(&mut self) {
        if self.status == SNat6Status::Disabled {
            let e = SNat6Event::StatusUpdated { status: SNat6Status::Enabled };
            self.process(e);
        }
    }

    pub fn disable(&mut self) {
        if self.status != SNat6Status::Disabled {
            let e = SNat6Event::StatusUpdated { status: SNat6Status::Disabled };
            self.process(e);
        }
    }

    pub fn to_nft(&self) -> Option<Vec<String>> { 
        None
    }

    // pub fn get_foreign_mappings<'a>(&'a self, mapping_ids: impl IntoIterator<Item = &'a Id>) -> Option<Vec<Id>> {
    //     let mut foreign_mappings = Vec::new();
    //     for mapping_id in mapping_ids {
    //         if self.mappings.get(mapping_id).is_none() {
    //             foreign_mappings.process(*mapping_id);
    //         }
    //     }
    //     if foreign_mappings.is_empty() {
    //         return None;
    //     }
    //     Some(foreign_mappings)
    // }

    // pub fn detect_foreign_mappings(&self, mapping_ids: &Vec<Id>) -> Result<(), SNatLogger> {
    //     let mut e = SNatLogger::new();

    //     for mapping_id in mapping_ids {
    //         let mapping = match self.mappings.get(mapping_id) {
    //             Some(value) => value,
    //             None => {
    //                 e += SNatError::MappingNotFoundInSNatEntry { id: self.id, mapping_id: *mapping_id };
    //             }
    //         };
    //     }
    //     if e.is_err() {
    //         Err(e);
    //     }
    //     Ok(())
    // }

    // pub async fn validate_for_egress<'a>(&'a mut self, mapping_ids: impl IntoIterator<Item = &'a Id>, min_count: usize) -> Result<(), SNatLogger> {
    //     let mut e = SNatLogger::new();
    //     // overlapping prefixes
    //     let mut observed = HashMap::<Ipv6Net, HashSet<&SNat6Target>>::new();
    //     for mapping_id in mapping_ids {
    //         let mapping = match self.mappings.get(mapping_id) {
    //             Some(value) => value,
    //             None => {
    //                 e += SNatError::MappingNotFoundInSNatEntry { mapping_id: *mapping_id, snat_id: self.id };
    //                 return Err(e);
    //             }
    //         };
    //         let prefix = mapping.target.compute_prefix().await;

    //         match observed.get_mut(&prefix) {
    //             Some(mappings) => { mappings.insert(mapping); },
    //             None => { observed.insert(prefix, HashSet::new()); }
    //         }
    //     }

    //     if observed.len() < min_count {
    //         e += SNatError::EgressRequiresMoreMappings { min_expected: min_count, found: observed.len() };
    //     }

    //     for (prefix, mappings) in observed {
    //         if mappings.len() > 1 {
    //             e += SNatError::MultipleMappingsResolveToSamePrefix { prefix, mappings: mappings.clone() }
    //         }
    //     }
    //     if e.is_err() {
    //         Err(e);
    //     }

    //     Ok(())
    // }
}

// #[derive(Debug)]
// pub struct TargetOverlap {
//     mapping_id: Id,
//     target_network: WanNetwork,
//     target_prefix: Ipv6Prefix
// }

// #[derive(Debug)]
// pub struct TargetOverlaps {
//     mapping_id: Id,
//     overlaps: Vec<TargetOverlap>
// }


pub struct ActiveSNatTargets {
    pub snat_id: Id,
    pub mapping_id: Id
    // pub snat_id: Id,
    // pub source: SNatSource,
    // pub source_prefix: Ipv6Prefix,
    // pub target: SNat6TargetPrefix,
    // pub target_prefix: Ipv6Prefix
}

// View:
// WanId -> Vec<>