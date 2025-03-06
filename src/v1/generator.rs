use super::domain;
use super::domain::Event;
use super::domain::Hadith;
use super::input_dtos::DayInputDto;
use super::input_dtos::EventInputDto;
use super::input_dtos::WeeklyHadithInputDto;
use super::output_dtos::*;
use super::params::WeekDay;
use crate::pathbuf;
use crate::prelude::Fallible;
use crate::v1::domain::DailyPrayerTime;
use crate::v1::domain::GregorianDate;
use chrono::offset::LocalResult;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Utc;
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
    pub weekly_hadith: HashMap<u16, Hadith>,
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
        let events = Self::make_events(year_dir.clone())?;
        let weekly_hadith = Self::make_weekly_hadiths(year_dir);
        let yearly_prayer_times =
            Self::make_yearly_prayer_times(year, input_dir_content.clone(), events)?;
        let this = Self {
            output_dir,
            yearly_prayer_times,
            weekly_hadith,
        };
        Ok(this)
    }

    fn make_events(year_dir: PathBuf) -> Fallible<HashMap<String, Event>> {
        let events = csv::Reader::from_path(pathbuf![year_dir, "events.csv"])?
            .deserialize()
            .flatten()
            .collect::<Vec<EventInputDto>>();
        Ok(events
            .into_iter()
            .map(|e| (e.date, Event { ar: e.ar, en: e.en }))
            .collect())
    }

    fn make_weekly_hadiths(year_dir: PathBuf) -> HashMap<u16, Hadith> {
        let Ok(mut reader) = csv::Reader::from_path(pathbuf![year_dir, "weekly_hadith.csv"]) else {
            return HashMap::new();
        };

        let hadiths = reader
            .deserialize()
            .flatten()
            .collect::<Vec<WeeklyHadithInputDto>>();
        hadiths
            .into_iter()
            .map(|h| {
                (
                    h.week,
                    Hadith {
                        hadith: h.hadith,
                        note: h.note,
                    },
                )
            })
            .collect()
    }

    fn make_yearly_prayer_times(
        year: u16,
        input_dir_map: HashMap<String, PathBuf>,
        year_events: HashMap<String, Event>,
    ) -> Fallible<Vec<DailyPrayerTime>> {
        let mut all_year = vec![];
        let mut days_count: u16 = 1;

        // loop over the 12 months
        for i in 1..=12 {
            if let Some(month_file) = input_dir_map.get(&format!("{i:02}.csv")) {
                let days = csv::Reader::from_path(month_file)?
                    .deserialize()
                    .flatten()
                    .collect::<Vec<DayInputDto>>();
                for day in days {
                    let event_key = format!("{}/{}", day.day, i);
                    let event = year_events.get(&event_key);
                    let daily_prayer_time = DailyPrayerTime {
                        gregorian_date: GregorianDate {
                            index: days_count,
                            day: day.day,
                            month: i,
                            year,
                        },
                        week_id: None,
                        hijri_date: day.hijri,
                        prayer_times: domain::PrayerTimes {
                            fajr: day.fajr,
                            sunrise: day.sunrise,
                            dhuhr: day.dhuhr,
                            asr: day.asr,
                            maghrib: day.maghrib,
                            ishaa: day.ishaa,
                        },
                        event: event.cloned(),
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
                .filter(|day| day.gregorian_date.month == i)
                .collect::<Vec<_>>();
            self.generate_day_idx(days)?;
        }
        Ok(())
    }

    pub fn generate_weekly_prayer_times(&mut self, starting_at: WeekDay) -> Fallible<()> {
        self.generate_week_idx(self.yearly_prayer_times.clone(), starting_at)
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
        let year_num = day_one.gregorian_date.year;
        let month_num = day_one.gregorian_date.month;
        let dir = pathbuf![
            self.output_dir.clone(),
            "day",
            year_num.to_string(),
            format!("{month_num:02}")
        ];
        fs::create_dir_all(&dir)?;
        for day in days_of_month {
            let day_path = pathbuf![dir.clone(), format!("{:02}.json", day.gregorian_date.day)];
            let day_file = File::create(day_path)?;
            let day_idx: DayOutputDto = day.into();
            let json = serde_json::to_value(&day_idx)?;
            serde_json::to_writer_pretty(day_file, &json)?;
        }
        Ok(())
    }

    fn generate_week_idx(
        &mut self,
        days_of_month: Vec<DailyPrayerTime>,
        starting_at: WeekDay,
    ) -> Fallible<()> {
        let Some(day_one) = days_of_month.first() else {
            return Ok(());
        };
        let year_num = day_one.gregorian_date.year;
        let week_dir = pathbuf![self.output_dir.clone(), "year", "weeks"];
        fs::create_dir_all(&week_dir)?;
        let week_path = pathbuf![week_dir.clone(), format!("{year_num}.json")];
        let week_file = File::create(week_path)?;
        let mut days_iter = days_of_month.into_iter().enumerate();
        let mut year_weeks = YearWeeksOutputDto {
            weeks: vec![],
            sha1: self.make_sha1()?,
        };

        for week_idx in 1..=53 {
            let id = format!("{year_num}{:02}", week_idx).parse()?;
            let hadith = self.weekly_hadith.get(&week_idx);
            let mut week = WeekOutputDto {
                id,
                mon: None,
                tue: None,
                wed: None,
                thu: None,
                fri: None,
                sat: None,
                sun: None,
                hadith: hadith.map(|h| HaidthOutputDto {
                    hadith: h.hadith.clone(),
                    note: h.note.clone(),
                }),
            };
            for _ in 0..7 {
                let Some((idx, day)) = days_iter.next() else {
                    break;
                };
                self.yearly_prayer_times[idx].week_id = Some(id);
                let LocalResult::Single(day_of_the_week) = Utc.with_ymd_and_hms(
                    day.gregorian_date.year.into(),
                    day.gregorian_date.month.into(),
                    day.gregorian_date.day.into(),
                    0,
                    0,
                    0,
                ) else {
                    break;
                };
                let day_of_the_week = day_of_the_week.weekday();
                match day_of_the_week {
                    chrono::Weekday::Mon => {
                        week.mon = Some(day.into());
                        if starting_at.previous() == WeekDay::Mon {
                            break;
                        }
                    }
                    chrono::Weekday::Tue => {
                        week.tue = Some(day.into());
                        if starting_at.previous() == WeekDay::Tue {
                            break;
                        }
                    }
                    chrono::Weekday::Wed => {
                        week.wed = Some(day.into());
                        if starting_at.previous() == WeekDay::Wed {
                            break;
                        }
                    }
                    chrono::Weekday::Thu => {
                        week.thu = Some(day.into());
                        if starting_at.previous() == WeekDay::Thu {
                            break;
                        }
                    }
                    chrono::Weekday::Fri => {
                        week.fri = Some(day.into());
                        if starting_at.previous() == WeekDay::Fri {
                            break;
                        }
                    }
                    chrono::Weekday::Sat => {
                        week.sat = Some(day.into());
                        if starting_at.previous() == WeekDay::Sat {
                            break;
                        }
                    }
                    chrono::Weekday::Sun => {
                        week.sun = Some(day.into());
                        if starting_at.previous() == WeekDay::Sun {
                            break;
                        }
                    }
                }
            }
            year_weeks.weeks.push(week);
        }
        let json = serde_json::to_value(&year_weeks)?;
        serde_json::to_writer_pretty(week_file, &json)?;
        Ok(())
    }

    fn generate_monthly_idx(&self, year: Vec<DailyPrayerTime>) -> Fallible<()> {
        let Some(day_one) = year.first() else {
            return Ok(());
        };
        let year_num = day_one.gregorian_date.year;
        let month_dir = pathbuf![self.output_dir.clone(), "month", year_num.to_string()];
        fs::create_dir_all(&month_dir)?;
        let days_iter = year.into_iter();
        for i in 1..=12 {
            let month = days_iter
                .clone()
                .filter(|day| day.gregorian_date.month == i)
                .map(|day| day.into())
                .collect::<Vec<DayOutputDto>>();
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
        let year_num = day_one.gregorian_date.year;
        let year_dir = pathbuf![self.output_dir.clone(), "year", "days"];
        fs::create_dir_all(&year_dir)?;

        let year_path = pathbuf![year_dir.clone(), format!("{year_num}.json")];
        let year_file = File::create(year_path)?;
        let days: Vec<DayOutputDto> = year.into_iter().map(Into::into).collect();
        let year = YearOutputDto {
            year: days,
            sha1: self.make_sha1()?,
        };
        let json = serde_json::to_value(&year)?;
        serde_json::to_writer_pretty(year_file, &json)?;
        Ok(())
    }

    pub fn generate_sha1(&self) -> Fallible<()> {
        let Some(day_one) = self.yearly_prayer_times.first() else {
            return Ok(());
        };
        let year_num = day_one.gregorian_date.year;
        let sha1_dir = pathbuf![self.output_dir.clone(), "sha1"];
        fs::create_dir_all(&sha1_dir)?;
        let sha1_path = pathbuf![sha1_dir, format!("{year_num}.json")];
        let sha1 = json!({
            "sha1": self.make_sha1()?,
        });
        let sha1_file = File::create(sha1_path)?;
        serde_json::to_writer_pretty(sha1_file, &sha1)?;
        Ok(())
    }

    fn make_sha1(&self) -> Fallible<String> {
        let mut hasher = Sha1::new();
        let yearly_prayer_times: Vec<DayOutputDto> = self
            .yearly_prayer_times
            .clone()
            .into_iter()
            .map(|day| day.into())
            .collect();
        let json = serde_json::to_string(&yearly_prayer_times)?;
        hasher.update(json);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}
