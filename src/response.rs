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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
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
        assert!(Response::parse(&[], &source()).is_none());
    }

    #[test]
    fn parse_some() {
        assert!(Response::parse(&BODY[0..22], &source()).is_none());
    }

    #[test]
    fn parse_almost_enough() {
        assert!(Response::parse(&BODY[0..55], &source()).is_none());
    }

    #[test]
    fn parse() {
        let response = Response::parse(&BODY, &source()).unwrap();

        assert_eq!(response.type_,    123);
        assert_eq!(response.code,     231);
        assert_eq!(response.ident,    31_719);
        assert_eq!(response.sequence, 59_259);

        match response.source {
            SockaddrInx::V6(_) => panic!(),
            SockaddrInx::V4(sockaddr_in) => {
                assert_eq!(sockaddr_in.sin_family,
                           libc::AF_INET as libc::sa_family_t);
                assert_eq!(sockaddr_in.sin_port, 0);
                assert_eq!(sockaddr_in.sin_addr.s_addr, 2_130_706_433);
                assert_eq!(sockaddr_in.sin_zero, [0, 0, 0, 0, 0, 0, 0, 0]);
            },
        }
    }
}
