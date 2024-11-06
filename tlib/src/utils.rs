#![allow(dead_code)]
use crate::{
    global::From,
    prelude::{FromBytes, ToBytes, ToValue},
    types::StaticType,
    values::FromValue,
    Type, Value,
};
use chrono::{DateTime, Local};
use std::{
    error::Error,
    sync::Mutex,
    time::{Duration, SystemTime},
};

const LONG_BIT: u32 = 64;
const UNIQUE_ID_BITS: u32 = 2;
const SEQUENCE_BITS: u32 = 16;
const TIMESTAMP_SHIFT_BITS: u32 = SEQUENCE_BITS + UNIQUE_ID_BITS;
const UNIQUE_ID_SHIFT_BITS: u32 = SEQUENCE_BITS;
const MAX_SEQUENCE_PER_MILLIS: u64 = 0xFFFFFFFFFFFF >> (LONG_BIT - SEQUENCE_BITS);
const UNIQUE_ID: u64 = 1;

static SEQUENCE: Mutex<u64> = Mutex::new(1);
static mut LAST_TIMESTAMP: u64 = 0;

/// Fetch a random global unique id by algorithm snowflake.
/// ## Usage
/// ```ignore
/// let id = SnowflakeGuidGenerator::next_id();
/// ```
pub struct SnowflakeGuidGenerator {}

impl SnowflakeGuidGenerator {
    pub fn next_id() -> Result<u64, Box<dyn Error>> {
        let mut sequence = SEQUENCE.lock()?;
        let mut timestamp = SnowflakeGuidGenerator::time_gen();

        unsafe {
            if timestamp == LAST_TIMESTAMP {
                *sequence += 1;
                if *sequence > MAX_SEQUENCE_PER_MILLIS {
                    timestamp = SnowflakeGuidGenerator::til_next_millis(timestamp);
                }
            }
            if timestamp > LAST_TIMESTAMP {
                *sequence = 0;
            }

            LAST_TIMESTAMP = timestamp;
        }

        Ok((timestamp << TIMESTAMP_SHIFT_BITS) | (UNIQUE_ID << UNIQUE_ID_SHIFT_BITS) | *sequence)
    }

    #[inline]
    fn time_gen() -> u64 {
        Timestamp::now().as_millis()
    }

    #[inline]
    fn til_next_millis(last_timestamp: u64) -> u64 {
        let mut timestamp: u64 = SnowflakeGuidGenerator::time_gen();
        while timestamp <= last_timestamp {
            timestamp = SnowflakeGuidGenerator::time_gen();
        }
        timestamp
    }
}

/// A struct to record a period of time and return it's time consumptions.
/// ## Usage
/// ```ignore
/// let recorder = TimeRecorder::new();
/// ...
/// let time_consumption = recorder.end();
/// ```
pub struct TimeRecorder {
    start: u64,
}

impl Default for TimeRecorder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TimeRecorder {
    #[inline]
    pub fn new() -> TimeRecorder {
        TimeRecorder {
            start: Timestamp::now().as_millis(),
        }
    }

    #[inline]
    pub fn end(&self) -> u64 {
        let end: u64 = Timestamp::now().as_millis();
        end - self.start
    }
}

impl From<u128> for u128 {
    fn from(t: u128) -> Self {
        t
    }
}
impl From<u128> for u64 {
    fn from(t: u128) -> Self {
        t as u64
    }
}
impl From<u128> for u32 {
    fn from(t: u128) -> Self {
        t as u32
    }
}
impl From<u128> for u16 {
    fn from(t: u128) -> Self {
        t as u16
    }
}

/// Get the timestamp since unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Timestamp(SystemTime);

impl Default for Timestamp {
    #[inline]
    fn default() -> Self {
        Self::now()
    }
}

impl Timestamp {
    #[inline]
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    #[inline]
    pub fn from_millis(millis: u128) -> Self {
        Self(SystemTime::UNIX_EPOCH + Duration::from_millis(millis as u64))
    }

    #[inline]
    pub fn from_micros(micros: u128) -> Self {
        Self(SystemTime::UNIX_EPOCH + Duration::from_micros(micros as u64))
    }

    #[inline]
    pub fn from_nanos(nanos: u128) -> Self {
        Self(SystemTime::UNIX_EPOCH + Duration::from_nanos(nanos as u64))
    }

    #[inline]
    pub fn as_millis<T: From<u128>>(&self) -> T {
        T::from(self.duration().as_millis())
    }

    #[inline]
    pub fn as_micros<T: From<u128>>(&self) -> T {
        T::from(self.duration().as_micros())
    }

    #[inline]
    pub fn as_nanos<T: From<u128>>(&self) -> T {
        T::from(self.duration().as_nanos())
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        let mut ts = (self.duration().as_millis() % 65536) as u16;
        if ts == u16::MAX {
            ts += 1;
        }
        ts
    }

    /// Default format: "%Y-%m-%d %H:%M:%S"
    ///
    /// See the [`chrono::format::strftime`] module for the whole supported escape sequences.
    #[inline]
    pub fn format_string(&self, format: Option<&str>) -> String {
        let format = format.unwrap_or("%Y-%m-%d %H:%M:%S");

        let date_time: DateTime<Local> = self.0.into();
        date_time.format(format).to_string()
    }
}

impl Timestamp {
    #[inline]
    fn duration(&self) -> Duration {
        self.0.duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }
}

impl StaticType for Timestamp {
    #[inline]
    fn static_type() -> Type {
        Type::from_name("Timestamp")
    }

    #[inline]
    fn bytes_len() -> usize {
        std::mem::size_of::<usize>()
    }
}

impl ToBytes for Timestamp {
    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        self.duration().as_nanos().to_bytes()
    }
}
impl FromBytes for Timestamp {
    #[inline]
    fn from_bytes(data: &[u8], len: usize) -> Self {
        let nanos = u128::from_bytes(data, len);
        Self::from_nanos(nanos)
    }
}
impl ToValue for Timestamp {
    #[inline]
    fn to_value(&self) -> Value {
        Value::new(self)
    }

    #[inline]
    fn value_type(&self) -> Type {
        Self::static_type()
    }
}
impl FromValue for Timestamp {
    #[inline]
    fn from_value(value: &Value) -> Self {
        Self::from_bytes(value.data(), Self::bytes_len())
    }
}

#[cfg(test)]
mod tests {
    use nohash_hasher::IntMap;

    use super::*;
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn test_snowflake_guid_generator() {
        let map: IntMap<u64, bool> = IntMap::default();
        let mut vec = vec![];
        let arc = Arc::new(Mutex::new(map));

        for _ in 0..5 {
            let arcm = arc.clone();
            vec.push(thread::spawn(move || {
                for _i in 0..500 {
                    let id = SnowflakeGuidGenerator::next_id().unwrap();
                    assert_ne!(0, id);
                    assert!(arcm.lock().unwrap().get(&id).is_none());
                    arcm.lock().unwrap().insert(id, true);
                }
            }));
        }

        for h in vec {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_timestamp_format_string() {
        let timestamp = Timestamp::now();
        println!("{}", timestamp.format_string(None));
    }

    #[test]
    fn test_timestamp_value() {
        let timestamp = Timestamp::now();
        let val = timestamp.to_value();
        assert_eq!(timestamp, val.get::<Timestamp>());
    }
}
