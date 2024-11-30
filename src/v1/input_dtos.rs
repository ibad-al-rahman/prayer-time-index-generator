use super::domain::DailyPrayerTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayInputDto {
    pub day: u8,
    pub hijri: String,
    pub fajer: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

impl From<DailyPrayerTime> for DayInputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            day: day.date.day,
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
