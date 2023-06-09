use crate::error::Error;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Timestamp {
    Start,
    End,
    Seconds(f64),
    Percentage(i32),
}

impl Timestamp {
    pub fn parse_timestamp(timestamp: &str) -> Result<Timestamp, Error> {
        match timestamp {
            t if TimestampRegex::match_seconds(t) => Timestamp::parse_seconds(t),
            t if TimestampRegex::match_hh_mm_ss(t) => Timestamp::parse_hh_mm_ss(t),
            t if TimestampRegex::match_percentage(t) => Timestamp::parse_percentage(t),
            _ => Err(Error::NoTimestampMatch),
        }
    }

    pub fn to_seconds(&self, video_length: f64) -> f64 {
        match self {
            Timestamp::Start => 0.0,
            Timestamp::End => video_length,
            Timestamp::Seconds(seconds) => *seconds,
            Timestamp::Percentage(percentage) => (video_length * *percentage as f64) / 100.0,
        }
    }

    pub fn is_before(&self, other: &Timestamp, video_length: f64) -> bool {
        match self {
            Timestamp::Start => match other {
                Timestamp::Start => false,
                Timestamp::End => true,
                Timestamp::Seconds(other_seconds) => *other_seconds > 0.0,
                Timestamp::Percentage(other_percent) => *other_percent > 0,
            },
            Timestamp::End => match other {
                Timestamp::End => false,
                Timestamp::Start => false,
                Timestamp::Seconds(other_seconds) => *other_seconds > video_length,
                Timestamp::Percentage(other_percent) => *other_percent > 100,
            },
            Timestamp::Percentage(self_percent) => match other {
                Timestamp::Start => false,
                Timestamp::End => true,
                Timestamp::Percentage(other_percent) => *other_percent > *self_percent,
                Timestamp::Seconds(other_seconds) => {
                    let self_seconds = (video_length * *self_percent as f64) / 100.0;
                    *other_seconds > self_seconds
                }
            },

            Timestamp::Seconds(self_seconds) => match other {
                Timestamp::Start => false,
                Timestamp::End => *self_seconds < video_length,
                Timestamp::Percentage(other_percent) => {
                    let other_seconds = (video_length * *other_percent as f64) / 100.0;
                    other_seconds > *self_seconds
                }
                Timestamp::Seconds(other_seconds) => other_seconds > self_seconds,
            },
        }
    }
}

impl Timestamp {
    fn parse_percentage(timestamp: &str) -> Result<Timestamp, Error> {
        if let Some(cap) = TimestampRegex::get_percentage_regex().captures(timestamp) {
            let percentage: i32 = cap.get(1).map(|x| x.as_str().parse().unwrap()).unwrap();
            if percentage > 100 {
                return Err(Error::PercentageOutOfRange(
                    "Percentage cannot be greater than 100%".to_owned(),
                ));
            }
            if percentage < 0 {
                return Err(Error::PercentageOutOfRange(
                    "Percentage cannot be lower than 0%".to_owned(),
                ));
            }
            return Ok(Timestamp::Percentage(percentage));
        }
        Err(Error::NoTimestampMatch)
    }

    fn parse_hh_mm_ss(timestamp: &str) -> Result<Timestamp, Error> {
        if let Some(cap) = TimestampRegex::get_hh_mm_ss_regex().captures(timestamp) {
            let hours = cap.get(1).map_or(0, |x| x.as_str().parse().unwrap());
            let minutes = cap.get(2).map_or(0, |x| x.as_str().parse().unwrap());
            let seconds = cap.get(3).map_or(0, |x| x.as_str().parse().unwrap());

            if minutes > 59 {
                return Err(Error::InvalidTime(
                    "Provided value for minutes cannot be greater than 60".to_owned(),
                ));
            }

            if seconds > 59 {
                return Err(Error::InvalidTime(
                    "Provided value for seconds cannot be greater than 60".to_owned(),
                ));
            }

            let total_seconds = (hours * 60 * 60 + minutes * 60 + seconds) as f64;
            return Ok(Timestamp::Seconds(total_seconds));
        }
        Err(Error::NoTimestampMatch)
    }

    fn parse_seconds(timestamp: &str) -> Result<Timestamp, Error> {
        if let Ok(seconds) = timestamp.parse::<f64>() {
            return Ok(Timestamp::Seconds(seconds));
        }
        Err(Error::NoTimestampMatch)
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
        let expected = Ok(Timestamp::Seconds(120.0));
        let result = Timestamp::parse_seconds(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_seconds_fails_with_invalid_input() {
        let test_timestamp = "12asdf";
        let expected = Err(Error::NoTimestampMatch);
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
        let expected = Err(Error::NoTimestampMatch);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_fails_with_percentage_higher_than_100() {
        let test_timestamp = "101%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Err(Error::PercentageOutOfRange(
            "Percentage cannot be greater than 100%".to_owned(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_percentage_fails_with_percentage_longer_than_3_digits() {
        let test_timestamp = "1101%";
        let result = Timestamp::parse_percentage(test_timestamp);
        let expected = Err(Error::NoTimestampMatch);
        assert_eq!(result, expected);
    }

    fn hh_mm_ss_to_seconds(hours: i32, minutes: i32, seconds: i32) -> f64 {
        (hours * 60 * 60 + minutes * 60 + seconds) as f64
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
        let expected = Err(Error::InvalidTime(
            "Provided value for minutes cannot be greater than 60".to_owned(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_hh_mm_ss_fails_with_invalid_seconds() {
        let test_timestamp = "45:60";
        let result = Timestamp::parse_hh_mm_ss(test_timestamp);
        let expected = Err(Error::InvalidTime(
            "Provided value for seconds cannot be greater than 60".to_owned(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_works_with_valid_seconds() {
        let test_timestamp = "120";
        let expected = Ok(Timestamp::Seconds(120.0));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_allows_up_to_nine_digits() {
        let test_timestamp = "123456789";
        let expected = Ok(Timestamp::Seconds(123456789.0));
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_timestamp_wont_allow_seconds_over_nine_digits() {
        let test_timestamp = "1234567891";
        let expected = Err(Error::NoTimestampMatch);
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
        let expected = Err(Error::NoTimestampMatch);
        let result = Timestamp::parse_timestamp(test_timestamp);
        assert_eq!(result, expected);
    }

    // is_before tests
    #[test]
    fn is_before_start_is_before_end() {
        let start = Timestamp::Start;
        let end = Timestamp::End;

        let result = start.is_before(&end, 100.0);
        assert_eq!(result, true);
    }

    #[test]
    fn is_before_end_is_not_before_start() {
        let start = Timestamp::Start;
        let end = Timestamp::End;

        let result = end.is_before(&start, 100.0);
        assert_eq!(result, false);
    }

    #[test]
    fn is_before_start_is_before_seconds() {
        let start = Timestamp::Start;
        let seconds = Timestamp::Seconds(10.0);

        let result = start.is_before(&seconds, 100.0);
        println!("{}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn is_before_seconds_is_not_before_start() {
        let start = Timestamp::Start;
        let seconds = Timestamp::Seconds(10.0);

        let result = seconds.is_before(&start, 100.0);
        println!("{}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn is_before_seconds_is_before_end() {
        let seconds = Timestamp::Seconds(10.0);
        let end = Timestamp::End;

        let result = seconds.is_before(&end, 100.0);
        println!("{}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn is_before_seconds_is_after_end() {
        let seconds = Timestamp::Seconds(101.0);
        let end = Timestamp::End;

        let result = seconds.is_before(&end, 100.0);
        assert_eq!(result, false);
    }

    #[test]
    fn is_before_percentage_is_after_start() {
        let percentage = Timestamp::Percentage(10);
        let start = Timestamp::Start;

        let result = percentage.is_before(&start, 100.0);
        assert_eq!(result, false);

        let result = start.is_before(&percentage, 100.0);
        assert_eq!(result, true);
    }

    #[test]
    fn is_before_percentage_is_before_end() {
        let percentage = Timestamp::Percentage(10);
        let end = Timestamp::End;

        let result = percentage.is_before(&end, 100.0);
        assert_eq!(result, true);

        let result = end.is_before(&percentage, 100.0);
        assert_eq!(result, false);
    }

    #[test]
    fn is_before_percentage_is_before_seconds() {
        let percentage = Timestamp::Percentage(5);
        let seconds = Timestamp::Seconds(10.0);

        let result = percentage.is_before(&seconds, 100.0);
        assert_eq!(result, true);

        let result = seconds.is_before(&percentage, 100.0);
        assert_eq!(result, false);
    }

    #[test]
    fn is_before_seconds_is_before_percentage() {
        let percentage = Timestamp::Percentage(20);
        let seconds = Timestamp::Seconds(10.0);

        let result = seconds.is_before(&percentage, 100.0);
        assert_eq!(result, true);

        let result = percentage.is_before(&seconds, 100.0);
        assert_eq!(result, false);
    }
}
