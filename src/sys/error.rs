#[derive(Debug)]
pub enum SysError {
    NotFound,
    Netlink(rtnetlink::Error)
}


impl From<rtnetlink::Error> for SysError {
    fn from(error: rtnetlink::Error) -> Self {
        SysError::Netlink(error)
    }
}