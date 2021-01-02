use std::{convert::TryInto, fmt, net::{IpAddr, Ipv4Addr, Ipv6Addr}};

#[derive(Clone, Copy)]
pub enum SockaddrInx {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6),
}

impl fmt::Debug for SockaddrInx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(_) => f.debug_struct("SockaddrInx::V4").finish(),
            Self::V6(_) => f.debug_struct("SockaddrInx::V6").finish(),
        }
    }
}

impl SockaddrInx {
    pub fn from_sockaddr(sockaddr: libc::sockaddr) -> Option<Self> {
        match sockaddr.sa_family as i32 {
            libc::AF_INET => Some(Self::V4(unsafe { *(
                &sockaddr as *const libc::sockaddr as *const libc::sockaddr_in
            ) })),
            libc::AF_INET6 => Some(Self::V6(unsafe { *(
                &sockaddr as *const libc::sockaddr as *const libc::sockaddr_in6
            ) })),
            _ => None,
        }
    }

    pub fn from_ip_addr(ip_addr: IpAddr) -> Self {
        match ip_addr {
            IpAddr::V4(ipv4_addr) => {
                let octets = ipv4_addr.octets();

                let s_addr =
                    ((octets[3] as u32) << 24) +
                    ((octets[2] as u32) << 16) +
                    ((octets[1] as u32) << 8)  +
                    ((octets[0] as u32));

                Self::V4(libc::sockaddr_in {
                    sin_family: libc::AF_INET as libc::sa_family_t,
                    sin_port: 0,
                    sin_addr: libc::in_addr { s_addr },
                    sin_zero: [0; 8],
                })
            }
            IpAddr::V6(ipv6_addr) => Self::V6(libc::sockaddr_in6 {
                sin6_family: libc::AF_INET6 as libc::sa_family_t,
                sin6_port: 0,
                sin6_flowinfo: 0,
                sin6_addr: libc::in6_addr { s6_addr: ipv6_addr.octets() },
                sin6_scope_id: 0,
            })
        }
    }

    pub fn to_ip_addr(&self) -> IpAddr {
        match self {
            Self::V4(sockaddr_in) => IpAddr::V4(Ipv4Addr::new(
                (sockaddr_in.sin_addr.s_addr)       as u8,
                (sockaddr_in.sin_addr.s_addr >> 8)  as u8,
                (sockaddr_in.sin_addr.s_addr >> 16) as u8,
                (sockaddr_in.sin_addr.s_addr >> 24) as u8,
            )),
            Self::V6(sockaddr_in6) => IpAddr::V6(Ipv6Addr::new(
                ((sockaddr_in6.sin6_addr.s6_addr[0] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[1] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[2] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[3] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[4] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[5] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[6] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[7] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[8] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[9] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[10] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[11] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[12] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[13] as u16)),
                ((sockaddr_in6.sin6_addr.s6_addr[14] as u16) << 8) +
                ((sockaddr_in6.sin6_addr.s6_addr[15] as u16)),
            )),
        }
    }

    pub fn sockaddr_ptr(&self) -> *const libc::sockaddr {
        match self {
            Self::V4(sockaddr_in) => sockaddr_in
                as *const libc::sockaddr_in as *const libc::sockaddr,
            Self::V6(sockaddr_in6) => sockaddr_in6
                as *const libc::sockaddr_in6 as *const libc::sockaddr,
        }
    }

    pub fn socklen(&self) -> libc::socklen_t {
        (match self {
            Self::V4(_) => std::mem::size_of::<libc::sockaddr_in>(),
            Self::V6(_) => std::mem::size_of::<libc::sockaddr_in6>(),
        }).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IPV4_BIG_ENDIAN: u32 = 16_777_343;
    const IPV6_BIG_ENDIAN: [u8; 16] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

    const IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const IPV6_ADDR: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

    #[test]
    fn from_to_ipv4_addr() {
        assert_eq!(
            SockaddrInx::from_ip_addr(IpAddr::V4(IPV4_ADDR)).to_ip_addr(),
            IPV4_ADDR,
        );
    }

    #[test]
    fn from_to_ipv6_addr() {
        assert_eq!(
            SockaddrInx::from_ip_addr(IpAddr::V6(IPV6_ADDR)).to_ip_addr(),
            IPV6_ADDR,
        );
    }

    #[test]
    fn from_ipv4_addr() {
        match SockaddrInx::from_ip_addr(IpAddr::V4(IPV4_ADDR)) {
            SockaddrInx::V6(_) => panic!(),
            SockaddrInx::V4(sockaddr_in) => {
                assert_eq!(sockaddr_in.sin_family,
                           libc::AF_INET as libc::sa_family_t);
                assert_eq!(sockaddr_in.sin_port, 0);
                assert_eq!(sockaddr_in.sin_addr.s_addr, IPV4_BIG_ENDIAN);
                assert_eq!(sockaddr_in.sin_zero, [0; 8]);
            },
        }
    }

    #[test]
    fn from_ipv6_addr() {
        match SockaddrInx::from_ip_addr(IpAddr::V6(IPV6_ADDR)) {
            SockaddrInx::V4(_) => panic!(),
            SockaddrInx::V6(sockaddr_in6) => {
                assert_eq!(sockaddr_in6.sin6_family,
                           libc::AF_INET6 as libc::sa_family_t);
                assert_eq!(sockaddr_in6.sin6_port, 0);
                assert_eq!(sockaddr_in6.sin6_flowinfo, 0);
                assert_eq!(sockaddr_in6.sin6_addr.s6_addr, IPV6_ADDR.octets());
                assert_eq!(sockaddr_in6.sin6_scope_id, 0);
            },
        }
    }

    #[test]
    fn to_ipv4_addr() {
        let sockaddr_inx = SockaddrInx::V4(libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: 0,
            sin_addr: libc::in_addr { s_addr: IPV4_BIG_ENDIAN },
            sin_zero: [0; 8],
        });

        match sockaddr_inx.to_ip_addr() {
            IpAddr::V6(_) => panic!(),
            IpAddr::V4(ipv4_addr) => assert_eq!(ipv4_addr, IPV4_ADDR),
        }
    }

    #[test]
    fn to_ipv6_addr() {
        let sockaddr_inx = SockaddrInx::V6(libc::sockaddr_in6 {
            sin6_family: libc::AF_INET6 as libc::sa_family_t,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: libc::in6_addr { s6_addr: IPV6_BIG_ENDIAN },
            sin6_scope_id: 0,
        });

        match sockaddr_inx.to_ip_addr() {
            IpAddr::V4(_) => panic!(),
            IpAddr::V6(ipv6_addr) => assert_eq!(ipv6_addr, IPV6_ADDR),
        }
    }
}
