use clap::Clap;

#[derive(Debug, Clap)]
#[clap(about, author, version)]
pub struct Options {
    #[clap(about = "The name or IP address of the destination host")]
    pub host: String,

    #[clap(
        short = 'f',
        long = "first",
        default_value = "1",
        about = "With what TTL to start",
    )]
    pub first_ttl: u8,

    #[clap(
        short = 'm',
        long = "max-hops",
        default_value = "30",
        about = "The maximum number of hops to probe",
    )]
    pub max_ttl: u8,

    #[clap(
        short = 'q',
        long = "queries",
        default_value = "3",
        about = "The number of probe packets per hop",
    )]
    pub nqueries: u16,

    #[clap(
        short = 'w',
        long = "wait",
        default_value = "5",
        about = "The time (in seconds) to wait for a response to a probe",
    )]
    pub waittime: u8,
}
