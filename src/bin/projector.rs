use anyhow::{Ok, Result};
use clap::Parser;
use projector_rust::{opts::Opts, config::Config};

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    Ok(())
}
