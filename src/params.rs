use crate::pathbuf;
use crate::v1::dtos::Generator as V1Generator;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum CliParams {
    /// V1 prayer times generator
    V1(V1Params),
}

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
}

impl V1Params {
    pub fn generate(&self) -> anyhow::Result<()> {
        let generator = V1Generator::new(
            self.year,
            self.year_dir.clone(),
            pathbuf![&self.output_dir_path, "v1"],
        )?;
        generator.generate_daily_prayer_times()?;
        generator.generate_weekly_prayer_times()?;
        Ok(())
    }
}
