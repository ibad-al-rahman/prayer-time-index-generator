use super::domain;
use super::input_dtos::DayInputDto;
use super::output_dtos::*;
use crate::pathbuf;
use crate::prelude::Fallible;
use crate::v1::domain::DailyPrayerTime;
use crate::v1::domain::MiladiDate;
use serde_json::json;
use sha1::Digest;
use sha1::Sha1;
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::fs::File;
use std::path::PathBuf;

pub struct Generator {
    pub output_dir: PathBuf,
    pub yearly_prayer_times: Vec<DailyPrayerTime>,
}

impl Generator {
    pub fn new(year: u16, year_dir: PathBuf, output_dir: PathBuf) -> Fallible<Self> {
        let input_dir_content = read_dir(&year_dir)?
            .flatten()
            .map(|entry| {
                (
                    entry.file_name().to_string_lossy().to_string(),
                    entry.path(),
                )
            })
            .collect::<HashMap<_, _>>();
        let yearly_prayer_times = Self::make_yearly_prayer_times(year, input_dir_content.clone())?;
        let this = Self {
            output_dir,
            yearly_prayer_times,
        };
        Ok(this)
    }

    fn make_yearly_prayer_times(
        year: u16,
        input_dir_map: HashMap<String, PathBuf>,
    ) -> Fallible<Vec<DailyPrayerTime>> {
        let mut all_year = vec![];
        let mut days_count: u16 = 1;

        // loop over the 12 months
        for i in 1..=12 {
            if let Some(month_file) = input_dir_map.get(&format!("{i}.csv")) {
                let days = csv::Reader::from_path(month_file)?
                    .deserialize()
                    .flatten()
                    .collect::<Vec<DayInputDto>>();
                for day in days {
                    let daily_prayer_time = DailyPrayerTime {
                        date: MiladiDate {
                            index: days_count,
                            day: day.day,
                            month: i,
                            year,
                        },
                        hijri_date: day.hijri,
                        prayer_times: domain::PrayerTimes {
                            fajer: day.fajer,
                            sunrise: day.sunrise,
                            dhuhr: day.dhuhr,
                            asr: day.asr,
                            maghrib: day.maghrib,
                            ishaa: day.ishaa,
                        },
                        event: None,
                    };
                    all_year.push(daily_prayer_time);
                    days_count += 1;
                }
            }
        }

        all_year.sort();

        Ok(all_year)
    }

    pub fn generate_daily_prayer_times(&self) -> Fallible<()> {
        for i in 1..=12 {
            let days = self
                .yearly_prayer_times
                .clone()
                .into_iter()
                .filter(|day| day.date.month == i)
                .collect::<Vec<_>>();
            self.generate_day_idx(days)?;
        }
        Ok(())
    }

    pub fn generate_weekly_prayer_times(&self) -> Fallible<()> {
        self.generate_week_idx(self.yearly_prayer_times.clone())
    }

    pub fn generate_monthly_prayer_times(&self) -> Fallible<()> {
        self.generate_monthly_idx(self.yearly_prayer_times.clone())
    }

    pub fn generate_yearly_prayer_times(&self) -> Fallible<()> {
        self.generate_year_idx(self.yearly_prayer_times.clone())
    }

    fn generate_day_idx(&self, days_of_month: Vec<DailyPrayerTime>) -> Fallible<()> {
        let Some(day_one) = days_of_month.first() else {
            return Ok(());
        };
        let year_num = day_one.date.year;
        let month_num = day_one.date.month;
        let dir = pathbuf![
            self.output_dir.clone(),
            "day",
            year_num.to_string(),
            format!("{month_num:02}")
        ];
        fs::create_dir_all(&dir)?;
        for day in days_of_month {
            let day_path = pathbuf![dir.clone(), format!("{:02}.json", day.date.day)];
            let day_file = File::create(day_path)?;
            let day_idx: DayOutputDto = day.into();
            let json = serde_json::to_value(&day_idx)?;
            serde_json::to_writer_pretty(day_file, &json)?;
        }
        Ok(())
    }

    fn generate_week_idx(&self, days_of_month: Vec<DailyPrayerTime>) -> Fallible<()> {
        let Some(day_one) = days_of_month.first() else {
            return Ok(());
        };
        let year_num = day_one.date.year;
        let week_dir = pathbuf![self.output_dir.clone(), "week", year_num.to_string()];
        fs::create_dir_all(&week_dir)?;
        let days_iter = days_of_month.into_iter();
        for week_idx in 1..=53 {
            let week = days_iter
                .clone()
                .skip((week_idx - 1) * 7)
                .take(7)
                .map(|day| day.into())
                .collect::<Vec<WeekDayOutputDto>>();
            let week_path = pathbuf![week_dir.clone(), format!("{week_idx:02}.json")];
            let week_file = File::create(week_path)?;
            let json = serde_json::to_value(&week)?;
            serde_json::to_writer_pretty(week_file, &json)?;
        }
        Ok(())
    }

    fn generate_monthly_idx(&self, year: Vec<DailyPrayerTime>) -> Fallible<()> {
        let Some(day_one) = year.first() else {
            return Ok(());
        };
        let year_num = day_one.date.year;
        let month_dir = pathbuf![self.output_dir.clone(), "month", year_num.to_string()];
        fs::create_dir_all(&month_dir)?;
        let days_iter = year.into_iter();
        for i in 1..=12 {
            let month = days_iter
                .clone()
                .filter(|day| day.date.month == i)
                .map(|day| day.into())
                .collect::<Vec<MonthDayOutputDto>>();
            let month_path = pathbuf![month_dir.clone(), format!("{i:02}.json")];
            let month_file = File::create(month_path)?;
            let json = serde_json::to_value(&month)?;
            serde_json::to_writer_pretty(month_file, &json)?;
        }
        Ok(())
    }

    fn generate_year_idx(&self, year: Vec<DailyPrayerTime>) -> Fallible<()> {
        let Some(day_one) = year.first() else {
            return Ok(());
        };
        let year_num = day_one.date.year;
        let year_dir = pathbuf![self.output_dir.clone(), "year"];
        fs::create_dir_all(&year_dir)?;

        let year_path = pathbuf![year_dir.clone(), format!("{year_num}.json")];
        let year_file = File::create(year_path)?;
        let year: Vec<YearlyOutputDto> = year.into_iter().map(Into::into).collect();
        let json = serde_json::to_value(&year)?;
        serde_json::to_writer_pretty(year_file, &json)?;
        Ok(())
    }

    pub fn generate_sha1(&self) -> Fallible<()> {
        let sha1_path = pathbuf![self.output_dir.clone(), "sha1.json"];
        let mut hasher = Sha1::new();
        let yearly_prayer_times: Vec<YearlyOutputDto> = self
            .yearly_prayer_times
            .clone()
            .into_iter()
            .map(|day| day.into())
            .collect();
        let json = serde_json::to_string(&yearly_prayer_times)?;
        hasher.update(json);
        let result = hasher.finalize();
        let sha1 = json!({
            "sha1": format!("{:x}", result)
        });
        let sha1_file = File::create(sha1_path)?;
        serde_json::to_writer_pretty(sha1_file, &sha1)?;
        Ok(())
    }
}
