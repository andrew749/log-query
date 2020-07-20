#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: Option<u32>,
}

impl Timestamp {
    pub fn new(year: &str, month: &str, day: &str, hour: &str, minute: &str, second: &str, millisecond: Option<&str>) -> Self {
        Timestamp {
            year: year.parse::<u32>().unwrap(),
            month: month.parse::<u32>().unwrap(),
            day: day.parse::<u32>().unwrap(),
            hour: hour.parse::<u32>().unwrap(),
            minute: minute.parse::<u32>().unwrap(),
            second: second.parse::<u32>().unwrap(),
            millisecond: match millisecond {
                Some(ms) => ms.parse::<u32>().ok(), 
                None => None, 
            },
        }
    }

    // Convert a match object in the form regex::Captures into a Timestamp
    pub fn from_match(captures: &regex::Captures) -> Self {
        Timestamp::new(
            &captures["year"],
            &captures["month"],
            &captures["day"],
            &captures["hour"],
            &captures["minute"],
            &captures["second"],
            match captures.name("millisecond") {
                Some(ms) => Some(ms.as_str()),
                None => None,
            },
        )
    }
}

