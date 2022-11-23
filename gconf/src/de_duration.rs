use once_cell::sync::Lazy;
use regex::Regex;
use serde::{de::Visitor, Deserializer};

static DURATION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"((?P<day>\d+)d)?((?P<hour>\d+)h)?((?P<min>\d+)m)?((?P<sec>\d+)s)?((?P<ms>\d+)ms)?"#,
    )
    .unwrap()
});
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

pub(crate) fn parse_duration<'de, D>(deserilizer: D) -> Result<std::time::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserilizer.deserialize_str(V)
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize, Debug)]
    struct Bar {
        #[serde(deserialize_with = "super::parse_duration")]
        dur: std::time::Duration,
    }
    #[test]
    fn foo() {
        let text = json!({
            "dur": "1h30m15s20ms"
        });
        let bar: Bar = serde_json::from_value(text).unwrap();
        let expect_dur = time::Duration::default()
            .add(time::Duration::hours(1))
            .add(time::Duration::minutes(30))
            .add(time::Duration::seconds(15))
            .add(time::Duration::milliseconds(20));
        assert_eq!(expect_dur, bar.dur);
    }
}
