use super::domain::DailyPrayerTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DayInputDto {
    pub day: u16,
    pub hijri: String,
    pub fajer: String,
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

impl From<DailyPrayerTime> for DayInputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            day: day.gregorian_date.day,
            hijri: day.hijri_date,
            fajer: day.prayer_times.fajer,
            sunrise: day.prayer_times.sunrise,
            dhuhr: day.prayer_times.dhuhr,
            asr: day.prayer_times.asr,
            maghrib: day.prayer_times.maghrib,
            ishaa: day.prayer_times.ishaa,
        }
    }
}
