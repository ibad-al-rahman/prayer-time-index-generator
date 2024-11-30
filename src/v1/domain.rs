use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyPrayerTime {
    pub date: MiladiDate,
    pub hijri_date: String,
    pub prayer_times: PrayerTimes,
    pub event: Option<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct PrayerTimes {
    pub fajer: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiladiDate {
    pub index: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Event {
    pub ar: String,
    pub en: String,
}

impl Display for MiladiDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}/{}", self.year, self.month, self.day))
    }
}

impl PartialOrd for MiladiDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl Ord for MiladiDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl Ord for DailyPrayerTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for DailyPrayerTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}
