use crate::pathbuf;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct PrayerTimes {
    fajr: String,
    sunrise: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Day {
    day: u8,
    hijri: String,
    prayer_times: PrayerTimes,
    events: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DayIndex {
    hijri: String,
    prayer_times: PrayerTimes,
    events: Vec<String>,
}

#[derive(Debug)]
struct Month {
    month_num: u8,
    days: Vec<Day>,
}

pub struct Generator {
    year: u16,
    output_dir: PathBuf,
    input_dir_map: HashMap<String, PathBuf>,
}

impl Generator {
    pub fn new(year: u16, year_dir: PathBuf, output_dir: PathBuf) -> anyhow::Result<Self> {
        let input_dir_content = read_dir(&year_dir)?
            .flatten()
            .map(|entry| {
                (
                    entry.file_name().to_string_lossy().to_string(),
                    entry.path(),
                )
            })
            .collect::<HashMap<_, _>>();
        let this = Self {
            year,
            output_dir,
            input_dir_map: input_dir_content,
        };
        Ok(this)
    }

    pub fn generate_daily_prayer_times(&self) -> anyhow::Result<()> {
        for i in 1..=12 {
            if let Some(month_file) = self.input_dir_map.get(&format!("{i}.json")) {
                let days: Vec<Day> = serde_json::from_reader(File::open(month_file)?)?;
                day_index_file(self.year, Month { month_num: i, days }, &self.output_dir)?;
            }
        }
        Ok(())
    }

    pub fn generate_weekly_prayer_times(&self) -> anyhow::Result<()> {
        let mut all_year = vec![];
        let mut days_count = 1;
        for i in 1..=12 {
            if let Some(month_file) = self.input_dir_map.get(&format!("{i}.json")) {
                let days: Vec<Day> = serde_json::from_reader(File::open(month_file)?)?;
                for mut day in days {
                    day.day = days_count;
                    all_year.push(day);
                    days_count += 1;
                }
            }
        }
        week_index_file(self.year, all_year, &self.output_dir)?;
        Ok(())
    }
}

fn day_index_file(year: u16, month: Month, output_dir: &PathBuf) -> anyhow::Result<()> {
    let month_num = month.month_num;
    let month_dir = pathbuf![
        output_dir,
        "day",
        year.to_string(),
        format!("{month_num:02}")
    ];
    fs::create_dir_all(&month_dir)?;

    for day in month.days {
        let day_path = pathbuf![month_dir.clone(), format!("{:02}.json", day.day)];
        let day_file = File::create(day_path)?;
        let day_idx = DayIndex {
            hijri: day.hijri,
            prayer_times: day.prayer_times,
            events: day.events,
        };
        let json = serde_json::to_value(&day_idx)?;
        serde_json::to_writer(day_file, &json)?;
    }

    Ok(())
}

fn week_index_file(year: u16, days: Vec<Day>, output_dir: &PathBuf) -> anyhow::Result<()> {
    let week_dir = pathbuf![output_dir, "week", year.to_string()];
    fs::create_dir_all(&week_dir)?;
    let days_iter = days.iter();
    for week_idx in 1..=53 {
        let week = days_iter
            .clone()
            .skip((week_idx - 1) * 7)
            .take(7)
            .collect::<Vec<_>>();
        let week_path = pathbuf![
            week_dir.clone(),
            format!("{:02}", format!("{week_idx}.json"))
        ];
        let week_file = File::create(week_path)?;
        let json = serde_json::to_value(&week)?;
        serde_json::to_writer(week_file, &json)?;
    }
    Ok(())
}
