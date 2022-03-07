use core::cmp;
use serde::de::Visitor;
use std::{fmt, ops::AddAssign};

#[derive(Clone)]
pub struct Time(chrono::NaiveDateTime);

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M").to_string())
    }
}

impl Time {
    const ISO_8601_FMT: &'static str = "%Y-%m-%dT%H:%M:%S%.fZ";

    /// cosntruct a new `Time` that holds the time at which it was created
    pub fn now() -> Self {
        Time(chrono::Utc::now().naive_utc())
    }

    /// Construct a new `Time` that represents this time, with an additional
    /// `duration` added on
    pub fn add(&self, duration: &chrono::Duration) -> Self {
        let mut time = self.clone();
        time.0.add_assign(*duration);
        time
    }

    /// return the ISO 8601 formatted time for this `Time` object (only UTC)
    pub fn as_iso_8601_string(&self) -> String {
        self.0.format(Self::ISO_8601_FMT).to_string()
    }

    /// Returns a reference to the `NaiveDateTime` that this struct wraps
    pub fn date_time(&self) -> &chrono::NaiveDateTime {
        &self.0
    }

    /// Returns a mutable reference to the `NaiveDateTime` that this struct wraps
    pub fn date_time_mut(&mut self) -> &mut chrono::NaiveDateTime {
        &mut self.0
    }
}

impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_iso_8601_string())
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeVisitor)
    }
}

struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a time parseable by `chrono::NaiveDatetime::parse_from_str`")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(
            match chrono::NaiveDateTime::parse_from_str(s, Time::ISO_8601_FMT) {
                Ok(t) => Time(t),
                Err(e) => {
                    println!("{:?}\n{}", s, e);
                    return Err(E::custom("cannot parse time string"));
                }
            },
        )
    }
}

impl cmp::PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
