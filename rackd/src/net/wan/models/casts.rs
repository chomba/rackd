use crate::{net::wan::models::*, util::models::EventData};

impl From<Created> for EventData {
    fn from(e: Created) -> Self {
        Self::Wan(WanEvent::Created(e))
    }
}

impl From<Renamed> for EventData {
    fn from(e: Renamed) -> Self {
        Self::Wan(WanEvent::Renamed(e))
    }
}

impl From<MacSetToAuto> for EventData {
    fn from(e: MacSetToAuto) -> Self {
        Self::Wan(WanEvent::MacSetToAuto(e))
    }
}

impl From<MacSetToSpoof> for EventData {
    fn from(e: MacSetToSpoof) -> Self {
        Self::Wan(WanEvent::MacSetToSpoof(e))
    }
}

impl From<Ipv6SetToRA> for EventData {
    fn from(e: Ipv6SetToRA) -> Self {
        Self::Wan(WanEvent::Ipv6SetToRA(e))
    }
}

impl From<Ipv6SetToStatic> for EventData {
    fn from(e: Ipv6SetToStatic) -> Self {
        Self::Wan(WanEvent::Ipv6SetToStatic(e))
    }
}

impl From<Ipv4SetToDHCP> for EventData {
    fn from(e: Ipv4SetToDHCP) -> Self {
        Self::Wan(WanEvent::Ipv4SetToDHCP(e))
    }
}

impl From<Ipv4SetToStatic> for EventData {
    fn from(e: Ipv4SetToStatic) -> Self {
        Self::Wan(WanEvent::Ipv4SetToStatic(e))
    }
}



// impl From<WanEvent> for EventData {
//     fn from(e: WanEvent) -> Self {
//         EventData::Wan(e)
//     }
// }

pub mod sqlite {
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
    use super::{WanConnection, WanMac};

    impl ToSql for WanMac {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(json.into())
        }
    }

    impl FromSql for WanMac {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let value: WanMac = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(value)
        }
    }

    impl ToSql for WanConnection {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            let json = serde_json::to_string(self).map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(json.into())
        }
    }

    impl FromSql for WanConnection {
        fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
            let value: WanConnection = serde_json::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))?;
            Ok(value)
        }
    }
}