use super::domain::DailyPrayerTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayOutputDto {
    pub id: u64,
    pub gregorian: String,
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

impl From<DailyPrayerTime> for DayOutputDto {
    fn from(day: DailyPrayerTime) -> Self {
        let hijri_date_components = day
            .hijri_date
            .splitn(3, '/')
            .flat_map(|c| c.parse::<u16>())
            .collect::<Vec<_>>();
        let hijri = if hijri_date_components.len() == 3 {
            format!(
                "{}/{:02}/{:02}",
                hijri_date_components[0], hijri_date_components[1], hijri_date_components[2]
            )
        } else {
            day.hijri_date
        };

        let id = match format!(
            "{}{:02}{:02}",
            day.gregorian_date.year, day.gregorian_date.month, day.gregorian_date.day
        )
        .parse()
        {
            Ok(id) => id,
            Err(_) => 0,
        };

        Self {
            id,
            gregorian: format!(
                "{}/{:02}/{:02}",
                day.gregorian_date.year, day.gregorian_date.month, day.gregorian_date.day
            ),
            hijri,
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
