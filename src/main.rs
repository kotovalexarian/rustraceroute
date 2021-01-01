mod checksum;
mod options;

use clap::{Clap};
use libc;
use options::{Options};

fn main() {
    let options = Options::parse();

    println!("{:?}", options);

    let socket = unsafe {
        libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP)
    };

    let sockaddr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: 0,
        sin_addr: libc::in_addr {
            s_addr: 0,
        },
        sin_zero: [0; 8],
    };

    let bind_result = unsafe {
        libc::bind(
            socket,
            &sockaddr as *const libc::sockaddr_in as *const libc::sockaddr,
            std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
        )
    };

    assert!(bind_result == 0);
}
