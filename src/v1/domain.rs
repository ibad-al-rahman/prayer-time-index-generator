use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyPrayerTime {
    pub gregorian_date: GregorianDate,
    pub hijri_date: String,
    pub prayer_times: PrayerTimes,
    pub week_id: Option<u64>,
    pub event: Option<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct PrayerTimes {
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub ishaa: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GregorianDate {
    pub index: u16,
    pub day: u16,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Event {
    pub ar: String,
    pub en: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Hadith {
    pub hadith: String,
    pub note: Option<String>,
}

impl Display for GregorianDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}/{}", self.year, self.month, self.day))
    }
}

impl PartialOrd for GregorianDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl Ord for GregorianDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl Ord for DailyPrayerTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gregorian_date.cmp(&other.gregorian_date)
    }
}

impl PartialOrd for DailyPrayerTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.gregorian_date.cmp(&other.gregorian_date))
    }
}
