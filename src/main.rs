use clap::{Clap};
use libc;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clap)]
#[clap(about, author, name = CRATE_NAME, version)]
struct Options {
    #[clap(about = "The name or IP address of the destination host")]
    host: String,

    #[clap(
        short = 'f',
        long = "first",
        default_value = "1",
        about = "With what TTL to start",
    )]
    first_ttl: u8,

    #[clap(
        short = 'm',
        long = "max-hops",
        default_value = "30",
        about = "The maximum number of hops to probe",
    )]
    max_ttl: u8,

    #[clap(
        short = 'q',
        long = "queries",
        default_value = "3",
        about = "The number of probe packets per hop",
    )]
    nqueries: u32,
}

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
