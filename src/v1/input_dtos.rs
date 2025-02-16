use super::domain::DailyPrayerTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DayInputDto {
    pub day: u16,
    pub hijri: String,
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Deserialize)]
pub struct EventInputDto {
    pub date: String,
    pub ar: String,
    pub en: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyHadithInputDto {
    pub week: u16,
    pub hadith: String,
}

impl From<DailyPrayerTime> for DayInputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            day: day.gregorian_date.day,
            hijri: day.hijri_date,
            fajr: day.prayer_times.fajr,
            sunrise: day.prayer_times.sunrise,
            dhuhr: day.prayer_times.dhuhr,
            asr: day.prayer_times.asr,
            maghrib: day.prayer_times.maghrib,
            ishaa: day.prayer_times.ishaa,
        }
    }
}
