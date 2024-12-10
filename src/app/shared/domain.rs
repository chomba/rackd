use serde::{Deserialize, Serialize};
use crate::{app::{isp::models::IspEvent, lan6::models::Lan6Event, snat6::models::SNat6Event, wan4::models::Wan4Event, wan6::models::Wan6Event}, util::domain::Id};

#[derive(Debug, Clone)]
pub struct Event {
    pub seq: u32,
    pub id: Id,
    pub stream_id: Id,
    pub data: InnerEvent,
    pub version: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InnerEvent {
    Lan6(Lan6Event),
    Wan4(Wan4Event),
    Wan6(Wan6Event),
    SNat6(SNat6Event),
    Isp(IspEvent)
}

impl From<Lan6Event> for InnerEvent {
    fn from(e: Lan6Event) -> Self {
        InnerEvent::Lan6(e)
    }
}

impl From<Wan4Event> for InnerEvent {
    fn from(e: Wan4Event) -> Self {
        InnerEvent::Wan4(e)
    }
}

impl From<Wan6Event> for InnerEvent {
    fn from(e: Wan6Event) -> Self {
        InnerEvent::Wan6(e)
    }
}

impl From<SNat6Event> for InnerEvent {
    fn from(e: SNat6Event) -> Self {
        InnerEvent::SNat6(e)
    }
}

impl From<IspEvent> for InnerEvent {
    fn from(e: IspEvent) -> Self {
        InnerEvent::Isp(e)
    }
}

#[derive(Debug)]
pub struct InnerEventCastError;

impl TryFrom<InnerEvent> for Lan6Event {
    type Error = InnerEventCastError;
    fn try_from(value: InnerEvent) -> Result<Self, Self::Error> {
        match value {
            InnerEvent::Lan6(e) => Ok(e),
            _ => Err(InnerEventCastError)
        }
    }
}

impl TryFrom<InnerEvent> for SNat6Event {
    type Error = InnerEventCastError;
    fn try_from(value: InnerEvent) -> Result<Self, Self::Error> {
        match value {
            InnerEvent::SNat6(e) => Ok(e),
            _ => Err(InnerEventCastError)
        }
    }
}
#[derive(Debug, Clone)]
pub struct Metadata {
    // Date: CreatedOn
    // Date: LastModified
    pub seq: u32,       // Sequence Number
    pub version: u32,   // Current Entity Version
    pub events: Option<Vec<Event>>
}

impl Default for Metadata  {
    fn default() -> Self {
        Self {
            seq: u32::default(),
            version: u32::default(),
            events: None
        }
    }
}

pub trait EventHandler<E> {
    fn handle(&mut self, event: &E) -> ();
}

// Rename EventSource to Aggregate
pub trait Entity where Self: Sized {
    type Event;
    fn id(&self) -> Id;
    fn apply(&mut self, e: &Self::Event);
    fn metadata(&mut self) -> &mut Metadata;

    fn events(&mut self) -> Vec<Event> {
        match self.metadata().events.take() {
            Some(events) => events,
            None => vec![]
        }
    }

    fn process(&mut self, e: Self::Event) where Self::Event: Into<InnerEvent> {
        self.apply(&e);
        let stream_id = self.id();
        let metadata = self.metadata();
        let e = Event {
            seq: u32::default(),
            id: Id::new(),
            stream_id: stream_id,
            data: e.into(),
            version: metadata.version + 1
        };
        match metadata.events {
            Some(ref mut events) => events.push(e),
            None => metadata.events = Some(vec![e])
        }
        metadata.version += 1;
    }
}
