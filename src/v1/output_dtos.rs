use super::domain::DailyPrayerTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct YearOutputDto {
    pub year: Vec<DayOutputDto>,
    pub sha1: String,
}

#[derive(Debug, Serialize)]
pub struct YearWeeksOutputDto {
    pub weeks: Vec<WeekOutputDto>,
    pub sha1: String,
}

#[derive(Debug, Serialize)]
pub struct WeekOutputDto {
    pub id: u64,
    pub mon: Option<DayOutputDto>,
    pub tue: Option<DayOutputDto>,
    pub wed: Option<DayOutputDto>,
    pub thu: Option<DayOutputDto>,
    pub fri: Option<DayOutputDto>,
    pub sat: Option<DayOutputDto>,
    pub sun: Option<DayOutputDto>,
    pub hadith: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayOutputDto {
    pub id: u64,
    pub gregorian: String,
    pub hijri: String,
    pub prayer_times: PrayerTimesOutputDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<EventOutputDto>,
}

#[derive(Debug, Serialize)]
pub struct PrayerTimesOutputDto {
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Serialize)]
pub struct EventOutputDto {
    pub ar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
}

impl From<DailyPrayerTime> for DayOutputDto {
    fn from(day: DailyPrayerTime) -> Self {
        let hijri_date_components = day
            .hijri_date
            .splitn(3, '/')
            .flat_map(|c| c.parse::<u16>())
            .collect::<Vec<_>>();
        let hijri = if hijri_date_components.len() == 3 {
            format!(
                "{:02}/{:02}/{}",
                hijri_date_components[0], hijri_date_components[1], hijri_date_components[2]
            )
        } else {
            day.hijri_date
        };

        // In the ID, we've made the format YYYYMMDD to make it easier to sort the dates
        let id = format!(
            "{}{:02}{:02}",
            day.gregorian_date.year, day.gregorian_date.month, day.gregorian_date.day
        )
        .parse()
        .unwrap_or_default();
        let event = match day.event {
            None => None,
            Some(e) => Some(EventOutputDto { ar: e.ar, en: e.en }),
        };

        Self {
            id,
            gregorian: format!(
                "{:02}/{:02}/{}",
                day.gregorian_date.day, day.gregorian_date.month, day.gregorian_date.year
            ),
            hijri,
            week_id: day.week_id,
            prayer_times: PrayerTimesOutputDto {
                fajr: day.prayer_times.fajr,
                sunrise: day.prayer_times.sunrise,
                dhuhr: day.prayer_times.dhuhr,
                asr: day.prayer_times.asr,
                maghrib: day.prayer_times.maghrib,
                ishaa: day.prayer_times.ishaa,
            },
            event,
        }
    }
}
