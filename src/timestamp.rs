use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Timestamp {
    Start,
    End,
    Seconds(i32),
    Percentage(i32),
}

#[derive(Debug, PartialEq)]
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
        Regex::new(r"^(\d{1,9})$").unwrap()
    }

    fn get_percentage_regex() -> Regex {
        Regex::new(r"^(\d{1,3})%").unwrap()
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
    fn parse_seconds_works_with_valid_input() {
        let test_timestamp = "120";
        let expected = Ok(Timestamp::Seconds(120));
        let result = Timestamp::parse_seconds(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_seconds_fails_with_invalid_input() {
        let test_timestamp = "12asdf";
        let expected = Err(TimestampError::NoMatch);
        let result = Timestamp::parse_seconds(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_works_with_valid_input() {
        let test_timestamp = "50%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Ok(Timestamp::Percentage(50));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_fails_with_percentage_lower_than_0() {
        let test_timestamp = "-2%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Err(TimestampError::NoMatch);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_fails_with_percentage_higher_than_100() {
        let test_timestamp = "101%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Err(TimestampError::PercentageOutOfRange(
            "Percentage cannot be greater than 100%".to_owned(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_fails_with_percentage_longer_than_3_digits() {
        let test_timestamp = "1101%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Err(TimestampError::NoMatch);
        assert_eq!(result, expected);
    }

    fn hh_mm_ss_to_seconds(hours: i32, minutes: i32, seconds: i32) -> i32 {
        hours * 60 * 60 + minutes * 60 + seconds
    }

    #[test]
    fn parse_hh_mm_ss_works_with_valid_input_with_hours() {
        let test_timestamp = "1:23:45";
        let expected_seconds = hh_mm_ss_to_seconds(1, 23, 45);
        let result = Timestamp::parse_hh_mm_ss(test_timestamp);
        let expected = Ok(Timestamp::Seconds(expected_seconds));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_hh_mm_ss_works_with_valid_input_without_hours() {
        let test_timestamp = "23:45";
        let expected_seconds = hh_mm_ss_to_seconds(0, 23, 45);
        let result = Timestamp::parse_hh_mm_ss(test_timestamp);
        let expected = Ok(Timestamp::Seconds(expected_seconds));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_hh_mm_ss_fails_with_invalid_minutes() {
        let test_timestamp = "60:45";
        let result = Timestamp::parse_hh_mm_ss(test_timestamp);
        let expected = Err(TimestampError::InvalidTime);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_hh_mm_ss_fails_with_invalid_seconds() {
        let test_timestamp = "45:60";
        let result = Timestamp::parse_hh_mm_ss(test_timestamp);
        let expected = Err(TimestampError::InvalidTime);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_works_with_valid_seconds() {
        let test_timestamp = "120";
        let expected = Ok(Timestamp::Seconds(120));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_allows_up_to_nine_digits() {
        let test_timestamp = "123456789";
        let expected = Ok(Timestamp::Seconds(123456789));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_wont_allow_seconds_over_nine_digits() {
        let test_timestamp = "1234567891";
        let expected = Err(TimestampError::NoMatch);
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_works_with_valid_percentage() {
        let test_timestamp = "100%";
        let expected = Ok(Timestamp::Percentage(100));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_works_with_hh_mm_ss() {
        let test_timestamp = "1:24";
        let expected_seconds = hh_mm_ss_to_seconds(0, 1, 24);
        let expected = Ok(Timestamp::Seconds(expected_seconds));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_fails_with_invalid_input() {
        let test_timestamp = "dsaf123:23:213%";
        let expected = Err(TimestampError::NoMatch);
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }
}
