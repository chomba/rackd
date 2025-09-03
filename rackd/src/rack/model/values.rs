use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Commands: Create(rackId, trunkNAME) / Rename(rackId, newName) / Set_Trunk_Interface(nodeId, newName) -> from nodeId get rackId
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct RackId(pub Uuid);

impl RackId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub enum RackStatus {
    Operational,
    Degraded,
    Offline
}

pub mod sqlite {
    use rusqlite::{Result, types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};
    use super::*;
    
    impl ToSql for RackId {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            Ok(self.0.to_string().into())
        }
    }
    
    impl FromSql for RackId {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let id = Uuid::try_from(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(Self(id))
        }
    }
}