use crate::{sockaddr_inx::SockaddrInx, request::Request};
use std::net::IpAddr;

#[derive(Debug)]
pub struct Response {
    pub source: IpAddr,
    pub type_: u8,
    pub code: u8,
    pub ident: u16,
    pub sequence: u16,
}

impl Response {
    pub fn parse(source: &SockaddrInx, body: &[u8]) -> Option<Self> {
        if body.len() < 2 * (20 /* IP header */ + 8 /* ICMP header */) {
            return None
        }

        Some(Self {
            source: source.to_ip_addr(),
            type_: body[20],
            code:  body[21],
            ident:    ((body[52] as u16) << 8) + (body[53] as u16),
            sequence: ((body[54] as u16) << 8) + (body[55] as u16),
        })
    }

    pub fn matches_request(&self, request: &Request) -> bool {
        self.ident == request.ident && self.sequence == request.sequence
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use super::*;

    const IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const IP_ADDR: IpAddr = IpAddr::V4(IPV4_ADDR);

    fn source() -> SockaddrInx { SockaddrInx::from_ip_addr(IP_ADDR) }

    const TYPE:     u8  = 123;
    const CODE:     u8  = 231;
    const IDENT:    u16 = 31_719;
    const SEQUENCE: u16 = 59_259;

    const BODY: [u8; 56] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        TYPE,
        CODE,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        123, 231, // IDENT
        231, 123, // SEQUENCE
    ];

    fn response() -> Response { Response::parse(&source(), &BODY).unwrap() }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", response()),
            "Response { source: 127.0.0.1, type_: 123, code: 231, ident: \
                31719, sequence: 59259 }",
        );
    }

    #[test]
    fn parse_empty() {
        assert!(Response::parse(&source(), &[]).is_none());
    }

    #[test]
    fn parse_some() {
        assert!(Response::parse(&source(), &BODY[0..22]).is_none());
    }

    #[test]
    fn parse_almost_enough() {
        assert!(Response::parse(&source(), &BODY[0..55]).is_none());
    }

    #[test]
    fn parse() {
        let response = response();

        assert_eq!(response.type_,    TYPE);
        assert_eq!(response.code,     CODE);
        assert_eq!(response.ident,    IDENT);
        assert_eq!(response.sequence, SEQUENCE);

        match response.source {
            IpAddr::V6(_) => panic!(),
            IpAddr::V4(ipv4_addr) => assert_eq!(ipv4_addr, IPV4_ADDR),
        }
    }

    #[test]
    fn matches_request() {
        assert!(response().matches_request(&Request::new(IDENT, SEQUENCE)));
    }

    #[test]
    fn does_not_match_request_ident() {
        assert!(!response().matches_request(&Request::new(IDENT + 1, SEQUENCE)));
    }

    #[test]
    fn does_not_match_request_sequence() {
        assert!(!response().matches_request(&Request::new(IDENT, SEQUENCE + 1)));
    }
}
