use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use std::fmt;

pub struct DateTimeWrapper {
    datetime: DateTime<FixedOffset>,
}

impl DateTimeWrapper {
    pub fn now() -> Self {
        Self {
            datetime: Local::now().into(),
        }
    }

    pub fn from_str(datetime: &str) -> Result<Self, chrono::ParseError> {
        let datetime = DateTime::parse_from_rfc3339(datetime)?;
        Ok(Self { datetime })
    }

    pub fn add_duration(&self, duration: Duration) -> Self {
        Self {
            datetime: self.datetime + duration,
        }
    }

    pub fn sub_duration(&self, duration: Duration) -> Self {
        Self {
            datetime: self.datetime - duration,
        }
    }

    pub fn format(&self, format: &str) -> String {
        self.datetime.format(format).to_string()
    }

    pub fn timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }

    pub fn set_timezone(&mut self, offset_seconds: i32) {
        self.datetime = self
            .datetime
            .with_timezone(&FixedOffset::east(offset_seconds));
    }

    pub fn php_format(&self, format: &str) -> String {
        let mut result = String::new();
        let mut chars = format.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                'd' => result.push_str(&format!("{:02}", self.datetime.day())),
                'D' => result.push_str(self.datetime.format("%a").to_string().as_str()),
                'j' => result.push_str(&format!("{}", self.datetime.day())),
                'l' => result.push_str(self.datetime.format("%A").to_string().as_str()),
                'N' => {
                    let wday = self.datetime.weekday().num_days_from_monday() + 1;
                    result.push_str(&format!("{}", wday));
                }
                'w' => result.push_str(&format!(
                    "{}",
                    self.datetime.weekday().num_days_from_sunday()
                )),
                'z' => result.push_str(&format!("{}", self.datetime.ordinal() - 1)),
                'F' => result.push_str(self.datetime.format("%B").to_string().as_str()),
                'm' => result.push_str(&format!("{:02}", self.datetime.month())),
                'M' => result.push_str(self.datetime.format("%b").to_string().as_str()),
                'n' => result.push_str(&format!("{}", self.datetime.month())),
                'Y' => result.push_str(&format!("{}", self.datetime.year())),
                'y' => result.push_str(&format!("{:02}", self.datetime.year() % 100)),
                'H' => result.push_str(&format!("{:02}", self.datetime.hour())),
                'i' => result.push_str(&format!("{:02}", self.datetime.minute())),
                's' => result.push_str(&format!("{:02}", self.datetime.second())),
                'u' => result.push_str(&format!("{:06}", self.datetime.timestamp_subsec_micros())),
                'v' => result.push_str(&format!("{:03}", self.datetime.timestamp_subsec_millis())),
                'O' => {
                    let offset_seconds = self.datetime.offset().local_minus_utc();
                    result.push_str(&format!(
                        "{:+03}{:02}",
                        offset_seconds / 3600,
                        (offset_seconds / 60) % 60
                    ));
                }
                'P' => {
                    let offset_seconds = self.datetime.offset().local_minus_utc();
                    result.push_str(&format!(
                        "{:+03}:{:02}",
                        offset_seconds / 3600,
                        (offset_seconds / 60) % 60
                    ));
                }
                'T' => result.push_str(self.datetime.format("%Z").to_string().as_str()),
                'Z' => result.push_str(&format!("{}", self.datetime.offset().local_minus_utc())),
                'c' => result.push_str(
                    self.datetime
                        .format("%Y-%m-%dT%H:%M:%S%:z")
                        .to_string()
                        .as_str(),
                ),
                'r' => result.push_str(
                    self.datetime
                        .format("%a, %d %b %Y %H:%M:%S %:z")
                        .to_string()
                        .as_str(),
                ),
                'U' => result.push_str(&format!("{}", self.datetime.timestamp())),
                _ => result.push(c),
            }
        }

        result
    }
}

impl fmt::Display for DateTimeWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.datetime)
    }
}
