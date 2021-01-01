use std::{convert::TryInto, net::IpAddr};

pub enum SockaddrInx {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6),
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
                    ((octets[0] as u32) << 24) +
                    ((octets[1] as u32) << 16) +
                    ((octets[2] as u32) << 8)  +
                    ((octets[3] as u32));

                Self::V4(libc::sockaddr_in {
                    sin_family: libc::AF_INET as libc::sa_family_t,
                    sin_port: 0,
                    sin_addr: libc::in_addr { s_addr },
                    sin_zero: [0; 8],
                })
            }
            IpAddr::V6(_) => unimplemented!(),
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
