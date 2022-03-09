use core::cmp;
use serde::de::Visitor;
use std::{fmt, ops::AddAssign, str::FromStr};

use super::error::BadArgumentError;

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

    /// Tries to parse a user input into a time
    /// Accepts `T+n`, `T + n`, `T`, etc.
    /// `n` may be
    /// `\d+w` for a number of weeks
    /// `\d+d` for a number of days
    /// `\d+h` for a number of hours
    /// `\d+m` for a number of minutes
    /// `\d+s` for a number of seconds
    pub fn try_parse_str(s: &str) -> Result<Self, BadArgumentError> {
        let err = |t: &str, reason: &str| -> Result<Self, BadArgumentError> {
            Err(BadArgumentError(format!(
                "`{}` is not a valid time -- {}",
                t, reason
            )))
        };

        let s = s.trim();
        let mut chars = s.chars();
        if s == "t" || s == "T" {
            return Ok(Time::now());
        }
        if {
            let zeroth = chars.nth(0);
            zeroth == Some('t') || zeroth == Some('T')
        } {
            enum Operation {
                Plus,
                Minus,
            }
            let mut operation = Operation::Plus;
            let mut operand = vec![];
            enum State {
                Operation,
                Operand,
            }
            let mut state = State::Operation;
            for char in chars {
                match char {
                    ' ' => {}
                    '+' | '-' => {
                        if let State::Operation = state {
                            if char == '-' {
                                operation = Operation::Minus;
                            }
                            state = State::Operand
                        } else {
                            return err(s, "you may only have one operator for relative times");
                        }
                    }
                    c => {
                        if let State::Operand = state {
                            operand.push(c);
                        } else {
                            return err(s, "you must include an operator, etiher '+' or '-'");
                        }
                    }
                }
            }
            // we now have an operand
            enum Unit {
                Weeks,
                Days,
                Hours,
                Minutes,
                Seconds,
            }
            let unit = if let Some(c) = operand.pop() {
                match c {
                    'w' | 'W' => Unit::Weeks,
                    'd' | 'D' => Unit::Days,
                    'h' | 'H' => Unit::Hours,
                    'm' | 'M' => Unit::Minutes,
                    's' | 'S' => Unit::Seconds,
                    _ => return err(s, &format!("'{}' is not a valid unit of time", c)),
                }
            } else {
                return err(s, "there is no operand");
            };

            let operand_str = operand.into_iter().collect::<String>();
            if let Ok(mut n) = operand_str.parse::<i64>() {
                if let Operation::Minus = operation {
                    n = -1 * n;
                }
                return Ok(Time::now().add(&match unit {
                    Unit::Weeks => chrono::Duration::weeks(n),
                    Unit::Days => chrono::Duration::days(n),
                    Unit::Hours => chrono::Duration::hours(n),
                    Unit::Minutes => chrono::Duration::minutes(n),
                    Unit::Seconds => chrono::Duration::seconds(n),
                }));
            } else {
                err(s, &format!("`{}` is not a valid integer", operand_str))
            }
        } else {
            let n_time = chrono::NaiveDateTime::parse_from_str(s, Self::ISO_8601_FMT);
            if let Ok(n_time) = n_time {
                Ok(Time(n_time))
            } else {
                err(s, "invalid date or time")
            }
        }
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
