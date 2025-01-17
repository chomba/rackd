use crate::{net::wan::models::*, util::models::{Event, EventData}};

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

impl From<MacSpoofed> for EventData {
    fn from(e: MacSpoofed) -> Self {
        Self::Wan(WanEvent::MacSpoofed(e))
    }
}

impl From<MacUnspoofed> for EventData {
    fn from(e: MacUnspoofed) -> Self {
        Self::Wan(WanEvent::MacUnspoofed(e))
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

impl From<Ipv6Disabled> for EventData {
    fn from(e: Ipv6Disabled) -> Self {
        Self::Wan(WanEvent::Ipv6Disabled(e))
    }
}



// impl From<WanEvent> for EventData {
//     fn from(e: WanEvent) -> Self {
//         EventData::Wan(e)
//     }
// }

pub mod sqlite {
    use rusqlite::{types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef}, Error, Result, ToSql};
    use crate::net::wan::models::WanConnection;

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