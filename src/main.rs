mod checksum;
mod options;
mod packet;
mod sockaddr_inx;

use clap::Clap;
use libc;
use options::Options;
use packet::Packet;
use sockaddr_inx::SockaddrInx;
use std::{convert::TryInto, net::IpAddr, time::{Duration, SystemTime}};

fn main() {
    let options = Options::parse();

    println!("{:?}", options);

    let host = match options.host.parse::<IpAddr>() {
        Ok(ip_addr) => ip_addr,
        Err(_) => unimplemented!(),
    };

    println!("{:?}", host);

    let socket = unsafe {
        libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP)
    };

    let mut reached_host = false;
    let mut current_ttl = options.first_ttl;

    while !reached_host && current_ttl <= options.max_ttl {
        println!("{}", current_ttl);

        for sequence in 0..options.nqueries {
            let packet = Packet::new(0, sequence);

            assert_eq!(0, unsafe { libc::setsockopt(
                socket,
                libc::IPPROTO_IP,
                libc::IP_TTL,
                &current_ttl as *const u8 as *const libc::c_void,
                std::mem::size_of::<u8>().try_into().unwrap(),
            ) });

            let traffic_class: u32 = 0;

            assert_eq!(0, unsafe { libc::setsockopt(
                socket,
                libc::IPPROTO_IP,
                libc::IP_TOS,
                &traffic_class as *const u32 as *const libc::c_void,
                std::mem::size_of::<i32>().try_into().unwrap(),
            ) });

            let message = packet.to_vec();

            let sockaddr_inx = SockaddrInx::from_ip_addr(host);

            assert_ne!(-1, unsafe { libc::sendto(
                socket,
                message.as_ptr() as *const libc::c_void,
                message.len(),
                0,
                sockaddr_inx.sockaddr_ptr(),
                sockaddr_inx.socklen(),
            ) });

            let timeval = libc::timeval { tv_sec: 2, tv_usec: 0 };

            assert_eq!(0, unsafe { libc::setsockopt(
                socket,
                libc::SOL_SOCKET,
                libc::SO_RCVTIMEO,
                &timeval as *const libc::timeval as *const libc::c_void,
                std::mem::size_of::<libc::timeval>().try_into().unwrap(),
            ) });

            let mut reply: Option<Reply> = None;

            let time_limit = SystemTime::now() + Duration::new(2, 0);

            while SystemTime::now() < time_limit {
                let response_body_data: [u8; 1024] = [0; 1024];

                let mut response_sockaddr_data = libc::sockaddr {
                    sa_family: 0,
                    sa_data: [0i8; 14],
                };

                let mut response_sockaddr_size: u32 =
                    std::mem::size_of::<libc::sockaddr>().try_into().unwrap();

                let response_body_size: isize = unsafe { libc::recvfrom(
                    socket,
                    response_body_data.as_ptr() as *mut libc::c_void,
                    std::mem::size_of::<[u8; 1024]>(),
                    0,
                    &mut response_sockaddr_data,
                    &mut response_sockaddr_size,
                ) };

                let response_sockaddr_inx = SockaddrInx::from_sockaddr(
                    response_sockaddr_data,
                );

                let tmp_reply = if response_body_size < 0 { None } else {
                    match &response_sockaddr_inx {
                        None => None,
                        Some(response_sockaddr_inx) => Reply::parse(
                            &response_body_data
                                [0..(response_body_size as usize)],
                            &response_sockaddr_inx,
                        ),
                    }
                };

                if let Some(tmp_reply) = tmp_reply {
                    if tmp_reply.ident == packet.ident &&
                        tmp_reply.sequence == packet.sequence
                    {
                        reply = Some(tmp_reply);
                        break
                    }
                }
            }
        }

        current_ttl += 1;
    }
}

#[derive(Debug)]
struct Reply {
    source: SockaddrInx,
    ident: u16,
    sequence: u16,
}

impl Reply {
    fn parse(body: &[u8], source: &SockaddrInx) -> Option<Self> {
        if body.len() < 2 * (8 + 20 /* IP header */) { return None }
        if body[20] /* type */ != 11 || body[21] /* code */ != 0 { return None }

        Some(Reply {
            source: *source,
            ident:    ((body[52] as u16) << 8) + (body[53] as u16),
            sequence: ((body[54] as u16) << 8) + (body[55] as u16),
        })
    }
}
