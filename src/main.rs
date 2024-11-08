mod generator;
mod params;
#[macro_use]
mod macros;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let params = params::CliParams::parse();
    let generator =
        generator::Generator::new(params.year, params.year_dir, params.output_dir_path)?;
    generator.generate_daily_prayer_times()?;
    generator.generate_weekly_prayer_times()?;
    Ok(())
}
