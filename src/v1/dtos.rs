use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub fajer: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub day: u8,
    pub hijri: String,
    pub fajer: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayIndex {
    pub hijri: String,
    pub prayer_times: PrayerTimes,
}

impl From<Day> for DayIndex {
    fn from(day: Day) -> Self {
        Self {
            hijri: day.hijri,
            prayer_times: PrayerTimes {
                fajer: day.fajer,
                sunrise: day.sunrise,
                dhuhr: day.dhuhr,
                asr: day.asr,
                maghrib: day.maghrib,
                ishaa: day.ishaa,
            },
        }
    }
}

#[derive(Debug)]
pub struct Month {
    pub month_num: u8,
    pub days: Vec<Day>,
}
