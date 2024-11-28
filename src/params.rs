use crate::v1::params::V1Params;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum CliParams {
    /// V1 prayer times generator
    V1(V1Params),
}
