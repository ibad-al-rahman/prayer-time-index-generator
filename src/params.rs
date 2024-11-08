use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliParams {
    /// The year to generate the prayer times for
    #[clap(short = 'y', long = "year")]
    pub year: u16,
    /// Path to directory containing 12 json files, each one of them contains the prayer times for
    /// the month
    #[clap(short = 'i', long = "input")]
    pub year_dir: PathBuf,
    /// Where to save the output directory
    #[clap(short = 'o', long = "output")]
    pub output_dir_path: PathBuf,
}
