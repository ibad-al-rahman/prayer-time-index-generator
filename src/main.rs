mod params;
#[macro_use]
mod macros;
mod prelude;
mod v1;

use clap::Parser;
use params::CliParams;
use prelude::Fallible;

fn main() -> Fallible<()> {
    match CliParams::try_parse()? {
        CliParams::V1(v1_params) => v1_params.generate()?,
    }
    Ok(())
}
