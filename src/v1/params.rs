use super::generator::Generator;
use crate::prelude::Fallible;
use clap::Parser;
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct V1Params {
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
    /// Input format
    #[clap(short = 'f', long = "format")]
    pub input_format: InputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum InputFormat {
    Json,
    Csv,
}

impl V1Params {
    pub fn generate(&self) -> Fallible<()> {
        let generator = Generator::new(
            self.year,
            self.year_dir.clone(),
            pathbuf![&self.output_dir_path, "v1"],
        )?;
        match self.input_format {
            InputFormat::Json => {}
            InputFormat::Csv => {
                generator.generate_daily_prayer_times()?;
                generator.generate_weekly_prayer_times()?;
                generator.generate_yearly_prayer_times()?;
                generator.generate_sha1()?;
            }
        }
        Ok(())
    }
}
