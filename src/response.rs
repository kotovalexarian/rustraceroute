use crate::sockaddr_inx::SockaddrInx;

#[derive(Debug)]
pub struct Response {
    pub source: SockaddrInx,
    pub type_: u8,
    pub code: u8,
    pub ident: u16,
    pub sequence: u16,
}

impl Response {
    pub fn parse(body: &[u8], source: &SockaddrInx) -> Option<Self> {
        if body.len() < 2 * (20 /* IP header */ + 8 /* ICMP header */) {
            return None
        }

        Some(Self {
            source: *source,
            type_: body[20],
            code:  body[21],
            ident:    ((body[52] as u16) << 8) + (body[53] as u16),
            sequence: ((body[54] as u16) << 8) + (body[55] as u16),
        })
    }
}
