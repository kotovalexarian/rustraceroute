use crate::sockaddr_inx::SockaddrInx;
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
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use super::*;

    const IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const IP_ADDR: IpAddr = IpAddr::V4(IPV4_ADDR);

    fn source() -> SockaddrInx { SockaddrInx::from_ip_addr(IP_ADDR) }

    const BODY: [u8; 56] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        123,
        231,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        123, 231, // 31_719
        231, 123, // 59_259
    ];

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
        let response = Response::parse(&source(), &BODY).unwrap();

        assert_eq!(response.type_,    123);
        assert_eq!(response.code,     231);
        assert_eq!(response.ident,    31_719);
        assert_eq!(response.sequence, 59_259);

        match response.source {
            IpAddr::V6(_) => panic!(),
            IpAddr::V4(ipv4_addr) => assert_eq!(
                ipv4_addr,
                Ipv4Addr::new(127, 0, 0, 1),
            ),
        }
    }
}
