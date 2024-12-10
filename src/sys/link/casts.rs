use futures::{TryStream, TryStreamExt};
use netlink_packet_route::link::{LinkAttribute, LinkFlag, LinkMessage};
use rtnetlink::Error;
use crate::sys::util::netlink::FromNetlinkMessage;
use super::models::*;

// IFF_UP (Admin Up) If not set then Link is Admin Down
// IFF_LOWER_UP (Link has Carrier/ Physical layer)
//  https://www.kernel.org/doc/Documentation/networking/operstates.txt

impl From<Vec<LinkFlag>> for LinkStatus {
    fn from(flags: Vec<LinkFlag>) -> Self {
        if let Ok(down) = LinkDown::try_from(&flags) {
            return LinkStatus::Down(down);
        } else if let Ok(up) = LinkUp::try_from(&flags) {
            return LinkStatus::Up(up);
        }
        LinkStatus::Unknwon
    }
}

impl TryFrom<&Vec<LinkFlag>> for LinkDown {
    type Error = ();
    fn try_from(flags: &Vec<LinkFlag>) -> Result<Self, Self::Error> {
        if !flags.contains(&LinkFlag::Up) {
            return Ok(LinkDown::AdminDown);
        } else if !flags.contains(&LinkFlag::LowerUp) {
            return Ok(LinkDown::Disconnected);
        }
        Err(())
    }
}

impl TryFrom<&Vec<LinkFlag>> for LinkUp {
    type Error = ();
    fn try_from(flags: &Vec<LinkFlag>) -> Result<Self, Self::Error> {
        if flags.contains(&LinkFlag::Up) && flags.contains(&LinkFlag::LowerUp)  {
            return Ok(LinkUp::Connected);
        }
        Err(())
    }
}

impl FromNetlinkMessage for Link {
    type Message = LinkMessage;
    async fn from_msg<T>(mut stream: T) -> Option<Self> where T: Unpin + TryStream<Ok = Self::Message, Error = Error> {
        let mut link = Link::default();
        let msg = match stream.try_next().await.unwrap() {
            Some(msg) => msg,
            None => return None
        };
    
        link.id = LinkId(msg.header.index);
        link.status = LinkStatus::from(msg.header.flags);

        for attribute in msg.attributes {
            match attribute {
                LinkAttribute::IfName(name) => link.name = LinkName(name),
                _ => { }
            }
        }
        Some(link)
    }
}