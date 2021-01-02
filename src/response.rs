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

    #[test]
    fn parse() {
        let ipv4_addr = Ipv4Addr::new(127, 0, 0, 1);
        let ip_addr = IpAddr::V4(ipv4_addr);
        let source = SockaddrInx::from_ip_addr(ip_addr);

        let body: [u8; 56] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            123,
            231,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            123, 231, // 31_719
            231, 123, // 59_259
        ];

        let response = Response::parse(&body, &source).unwrap();

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
