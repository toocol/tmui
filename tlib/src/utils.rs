#![allow(dead_code)]
use std::{error::Error, sync::Mutex, time::SystemTime};

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

    fn time_gen() -> u64 {
        TimeStamp::timestamp()
    }

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

impl TimeRecorder {
    pub fn new() -> TimeRecorder {
        TimeRecorder {
            start: TimeStamp::timestamp(),
        }
    }

    pub fn end(&self) -> u64 {
        let end = TimeStamp::timestamp();
        end - self.start
    }
}

pub struct TimeStamp {}

impl TimeStamp {
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    pub fn timestamp_micros() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }

    pub fn timestamp_16() -> u16 {
        let mut ts = (TimeStamp::timestamp() % 65536) as u16;
        if ts == u16::MAX {
            ts += 1;
        }
        ts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn test_snowflake_guid_generator() {
        let map: HashMap<u64, bool> = HashMap::new();
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
}
