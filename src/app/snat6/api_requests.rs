use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSNat {
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSNatTarget {
    pub id: String,
    pub network: String
}

pub struct UpdateSNatTarget {
    pub id: String,
    pub mapping_id: String,
    pub network: String
}

pub struct EnableSNatSingleEgress {
    pub id: String,
    pub mapping: String
} 

pub struct EnableSNatFailoverEgress {
    pub id: String,
    pub mappings: Vec<String>
}

pub struct EnableSNatLoadsharedEgress {
    pub id: String,
    pub mappings: Vec<String>
}

pub struct DisableSNatEgress {
    pub id: String
}