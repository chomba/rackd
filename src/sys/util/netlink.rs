use futures::TryStream;
use rtnetlink::Handle;
// use rtnetlink::Handle;

pub trait FromNetlinkMessage where Self: Sized {
    type Message;
    async fn from_msg<T>(stream: T) -> Option<Self> where T: Unpin + TryStream<Ok = Self::Message, Error = rtnetlink::Error>;
}

pub trait NlQuery {
    type Ok;
    type Err;
    async fn run(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err>;
}

pub trait NlCommand {
    type Ok;
    type Err;
    async fn exec(self, netlink: &Netlink) -> Result<Self::Ok, Self::Err>;
}

#[derive(Clone)]
pub struct Netlink {
    route: Handle
}

impl Netlink {
    pub fn connect() -> std::io::Result<Netlink> {
        let (connection, route_handle, _) = rtnetlink::new_connection()?;
        tokio::spawn(connection);
        let netlink = Self {
            route: route_handle
        };
        Ok(netlink)
    }

    pub fn route(&self) -> Handle {
        self.route.clone()
    }

    pub async fn run<Q>(&self, query: Q) -> Result<Q::Ok, Q::Err> where Q: NlQuery {
        query.run(self).await
    }

    pub async fn exec<C>(&self, cmd: C) -> Result<C::Ok, C::Err> where C: NlCommand {
        cmd.exec(self).await
    }
}