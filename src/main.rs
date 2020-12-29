use clap::{Clap};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clap)]
#[clap(about, author, name = CRATE_NAME, version)]
struct Options {
    #[clap(about = "The name or IP address of the destination host")]
    host: String,
}

fn main() {
    let options = Options::parse();

    println!("host: {:?}", options.host);
}
