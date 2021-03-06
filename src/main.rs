mod checksum;
mod options;
mod request;
mod response;
mod sockaddr_inx;

use clap::Clap;
use libc;
use options::Options;
use request::Request;
use response::Response;
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
        let current_address = iterate_ttl(
            &options,
            &host,
            socket,
            current_ttl,
        );

        if let Some(ip_addr) = current_address {
            println!("{} {}", current_ttl, ip_addr);
        }
        else {
            println!("{} ***", current_ttl);
        }

        current_ttl += 1;
    }
}

fn iterate_ttl(
    options: &Options,
    host: &IpAddr,
    socket: libc::c_int,
    current_ttl: u8,
) -> Option<IpAddr> {
    for sequence in 0..options.nqueries {
        let request = Request::new(0, sequence);

        send_request(socket, current_ttl, &host, &request);

        set_timeout(socket, 2, 0);

        if let Some(response) = recv_response1(socket, &request) {
            if response.type_ != 11 || response.code != 0 {
                continue
            }

            return Some(response.source)
        }
    }

    return None
}

fn send_request(
    socket: libc::c_int,
    current_ttl: u8,
    host: &IpAddr,
    request: &Request,
) {
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

    let message = request.to_vec();

    let sockaddr_inx = SockaddrInx::from_ip_addr(*host);

    assert_ne!(-1, unsafe { libc::sendto(
        socket,
        message.as_ptr() as *const libc::c_void,
        message.len(),
        0,
        sockaddr_inx.sockaddr_ptr(),
        sockaddr_inx.socklen(),
    ) });
}

fn set_timeout(socket: libc::c_int, sec: i64, usec: i64) {
    let timeval = libc::timeval { tv_sec: sec, tv_usec: usec };

    assert_eq!(0, unsafe { libc::setsockopt(
        socket,
        libc::SOL_SOCKET,
        libc::SO_RCVTIMEO,
        &timeval as *const libc::timeval as *const libc::c_void,
        std::mem::size_of::<libc::timeval>().try_into().unwrap(),
    ) });
}

fn recv_response1(socket: libc::c_int, request: &Request) -> Option<Response> {
    let time_limit = SystemTime::now() + Duration::new(2, 0);

    while SystemTime::now() < time_limit {
        if let Some(tmp_response) = recv_response2(socket) {
            if tmp_response.does_match_request(request) {
                return Some(tmp_response)
            }
        }
    }

    return None
}

fn recv_response2(socket: libc::c_int) -> Option<Response> {
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

    if response_body_size < 0 { None } else {
        match &response_sockaddr_inx {
            None => None,
            Some(response_sockaddr_inx) => Response::parse(
                &response_sockaddr_inx,
                &response_body_data
                    [0..(response_body_size as usize)],
            ),
        }
    }
}
