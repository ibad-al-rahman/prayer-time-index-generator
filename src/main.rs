mod params;
#[macro_use]
mod macros;
mod v1;

use clap::Parser;
use params::CliParams;

fn main() -> anyhow::Result<()> {
    match CliParams::try_parse()? {
        CliParams::V1(v1_params) => v1_params.generate()?,
    }
    Ok(())
}
