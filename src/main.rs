mod checksum;
mod options;
mod packet;
mod sockaddr_inx;

use clap::Clap;
use libc;
use options::Options;
use packet::Packet;
use sockaddr_inx::SockaddrInx;
use std::{convert::TryInto, net::IpAddr};

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

            let message: Vec<u8> = packet.into();

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

            let mut response: [u8; 1024] = [0; 1024];

            let response_size: isize = unsafe { libc::recvfrom(
                socket,
                response.as_ptr() as *mut libc::c_void,
                std::mem::size_of::<[u8; 1024]>(),
                0,
                0 as *mut libc::sockaddr,
                0 as *mut u32,
            ) };
        }

        current_ttl += 1;
    }
}
