use regex::Regex;

#[derive(Debug)]
pub enum Timestamp {
    Start,
    End,
    Seconds(i32),
    Percentage(i32),
}

#[derive(Debug)]
pub enum TimestampError {
    PercentageOutOfRange(String),
    InvalidTime,
    NoMatch,
}

impl Timestamp {
    pub fn parse_timestamp(timestamp: &str) -> Result<Timestamp, TimestampError> {
        match timestamp {
            t if TimestampRegex::match_seconds(t) => Timestamp::parse_seconds(t),
            t if TimestampRegex::match_hh_mm_ss(t) => Timestamp::parse_hh_mm_ss(t),
            t if TimestampRegex::match_percentage(t) => Timestamp::parse_percentage(t),
            _ => Err(TimestampError::NoMatch),
        }
    }
}

impl Timestamp {
    fn parse_percentage(timestamp: &str) -> Result<Timestamp, TimestampError> {
        if let Some(cap) = TimestampRegex::get_percentage_regex().captures(timestamp) {
            let percentage: i32 = cap.get(1).map(|x| x.as_str().parse().unwrap()).unwrap();
            if percentage > 100 {
                return Err(TimestampError::PercentageOutOfRange(
                    "Percentage cannot be greater than 100%".to_owned(),
                ));
            }
            if percentage < 0 {
                return Err(TimestampError::PercentageOutOfRange(
                    "Percentage cannot be lower than 0%".to_owned(),
                ));
            }
            return Ok(Timestamp::Percentage(percentage));
        }
        Err(TimestampError::NoMatch)
    }

    fn parse_hh_mm_ss(timestamp: &str) -> Result<Timestamp, TimestampError> {
        if let Some(cap) = TimestampRegex::get_hh_mm_ss_regex().captures(timestamp) {
            // There is always gonna be 3 capture is is an option type
            let hours = cap.get(1).map_or(0, |x| x.as_str().parse().unwrap());
            let minutes = cap.get(2).map_or(0, |x| x.as_str().parse().unwrap());
            let seconds = cap.get(3).map_or(0, |x| x.as_str().parse().unwrap());

            if minutes > 59 || seconds > 59 {
                return Err(TimestampError::InvalidTime);
            }

            let total_seconds = hours * 60 * 60 + minutes * 60 + seconds;
            return Ok(Timestamp::Seconds(total_seconds));
        }
        Err(TimestampError::NoMatch)
    }

    fn parse_seconds(timestamp: &str) -> Result<Timestamp, TimestampError> {
        if let Ok(seconds) = timestamp.parse::<i32>() {
            return Ok(Timestamp::Seconds(seconds));
        }
        Err(TimestampError::NoMatch)
    }
}

struct TimestampRegex;
impl TimestampRegex {
    fn get_hh_mm_ss_regex() -> Regex {
        Regex::new(r"^(?:(\d{1,2}):)?(\d{1,2}):(\d{1,2})$").unwrap()
    }

    fn get_seconds_regex() -> Regex {
        Regex::new(r"^(\d{1,3})%$").unwrap()
    }

    fn get_percentage_regex() -> Regex {
        Regex::new(r"^(\d{1,9}%)").unwrap()
    }

    fn match_hh_mm_ss(timestamp: &str) -> bool {
        TimestampRegex::get_hh_mm_ss_regex().is_match(timestamp)
    }

    fn match_seconds(timestamp: &str) -> bool {
        TimestampRegex::get_seconds_regex().is_match(timestamp)
    }

    fn match_percentage(timestamp: &str) -> bool {
        TimestampRegex::get_percentage_regex().is_match(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        todo!();
    }
}
