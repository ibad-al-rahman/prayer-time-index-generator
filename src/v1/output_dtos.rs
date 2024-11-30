use super::domain::DailyPrayerTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayOutputDto {
    pub hijri: String,
    pub prayer_times: PrayerTimesOutputDto,
}

#[derive(Debug, Serialize)]
pub struct PrayerTimesOutputDto {
    pub fajer: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekDayOutputDto {
    pub day_index: u8,
    pub hijri: String,
    pub prayer_times: PrayerTimesOutputDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearlyOutputDto {
    pub day_index: u8,
    pub hijri: String,
    pub prayer_times: PrayerTimesOutputDto,
}

impl From<DailyPrayerTime> for DayOutputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            hijri: day.hijri_date,
            prayer_times: PrayerTimesOutputDto {
                fajer: day.prayer_times.fajer,
                sunrise: day.prayer_times.sunrise,
                dhuhr: day.prayer_times.dhuhr,
                asr: day.prayer_times.asr,
                maghrib: day.prayer_times.maghrib,
                ishaa: day.prayer_times.ishaa,
            },
        }
    }
}

impl From<DailyPrayerTime> for WeekDayOutputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            day_index: day.date.index,
            hijri: day.hijri_date,
            prayer_times: PrayerTimesOutputDto {
                fajer: day.prayer_times.fajer,
                sunrise: day.prayer_times.sunrise,
                dhuhr: day.prayer_times.dhuhr,
                asr: day.prayer_times.asr,
                maghrib: day.prayer_times.maghrib,
                ishaa: day.prayer_times.ishaa,
            },
        }
    }
}

impl From<DailyPrayerTime> for YearlyOutputDto {
    fn from(day: DailyPrayerTime) -> Self {
        Self {
            day_index: day.date.index,
            hijri: day.hijri_date,
            prayer_times: PrayerTimesOutputDto {
                fajer: day.prayer_times.fajer,
                sunrise: day.prayer_times.sunrise,
                dhuhr: day.prayer_times.dhuhr,
                asr: day.prayer_times.asr,
                maghrib: day.prayer_times.maghrib,
                ishaa: day.prayer_times.ishaa,
            },
        }
    }
}
