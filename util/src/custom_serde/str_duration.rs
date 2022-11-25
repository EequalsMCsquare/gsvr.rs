use once_cell::sync::Lazy;
use regex::Regex;
use serde::{de::Visitor, Deserialize, Deserializer};
use time::error::ConversionRange;

static DURATION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"((?P<day>\d+)d)?((?P<hour>\d+)h)?((?P<min>\d+)m)?((?P<sec>\d+)s)?((?P<ms>\d+)ms)?$"#,
    )
    .unwrap()
});

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct StrDuration(std::time::Duration);

impl<'de> Deserialize<'de> for StrDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(V).map(|s| StrDuration(s))
    }
}

impl Into<std::time::Duration> for StrDuration {
    fn into(self) -> std::time::Duration {
        self.0
    }
}

impl TryInto<time::Duration> for StrDuration {
    type Error = ConversionRange;

    fn try_into(self) -> Result<time::Duration, Self::Error> {
        self.0.try_into()
    }
}

impl From<std::time::Duration> for StrDuration {
    fn from(dur: std::time::Duration) -> Self {
        Self(dur)
    }
}

impl TryFrom<time::Duration> for StrDuration {
    type Error = ConversionRange;

    fn try_from(value: time::Duration) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

struct V;

impl<'de> Visitor<'de> for V {
    type Value = std::time::Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("exampel format 3d27h33m50s234ms")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let capture = match DURATION_REGEX.captures(v) {
            Some(capture) => capture,
            None => return Ok(Default::default()),
        };
        let mut dur = time::Duration::default();

        // why unwrap? 用正则\d捕捉的一定是数字
        if let Some(day) = capture.name("day") {
            dur += time::Duration::days(day.as_str().parse().unwrap())
        }
        if let Some(hour) = capture.name("hour") {
            dur += time::Duration::hours(hour.as_str().parse().unwrap())
        }
        if let Some(min) = capture.name("min") {
            dur += time::Duration::minutes(min.as_str().parse().unwrap())
        }
        if let Some(sec) = capture.name("sec") {
            dur += time::Duration::seconds(sec.as_str().parse().unwrap())
        }
        if let Some(ms) = capture.name("ms") {
            dur += time::Duration::milliseconds(ms.as_str().parse().unwrap())
        }
        Ok(dur.try_into().unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct Bar {
        dur: super::StrDuration,
    }
    #[test]
    fn parse_str_duration_1() {
        let text = json!({
            "dur": "1h30m15s20ms"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::default()
            .add(time::Duration::hours(1))
            .add(time::Duration::minutes(30))
            .add(time::Duration::seconds(15))
            .add(time::Duration::milliseconds(20));
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }

    #[test]
    fn parse_str_duration_2() {
        let text = json!({
            "dur": "1d25h61m61s1001ms"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::default()
            .add(time::Duration::days(1))
            .add(time::Duration::hours(25))
            .add(time::Duration::minutes(61))
            .add(time::Duration::seconds(61))
            .add(time::Duration::milliseconds(1001));
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }

    #[test]
    fn parse_str_duration_3() {
        let text = json!({
            "dur": "11d"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::days(11);
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }

    #[test]
    fn parse_str_duration_4() {
        let text = json!({
            "dur": "23h"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::hours(23);
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }
    #[test]
    fn parse_str_duration_5() {
        let text = json!({
            "dur": "59m"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::minutes(59);
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }

    #[test]
    fn parse_str_duration_6() {
        let text = json!({
            "dur": "59s"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::seconds(59);
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }

    #[test]
    fn parse_str_duration_7() {
        let text = json!({
            "dur": "999ms"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::milliseconds(999);
        assert_eq!(expect_dur.try_into(), Ok(bar.dur));
    }
}
