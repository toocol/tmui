use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
    thread::ThreadId,
    time::Instant,
};

const TRACK_FILE_NAME: &str = "track_file";

static TRACKED: AtomicBool = AtomicBool::new(false);
#[inline]
pub(crate) fn set_tracked() {
    TRACKED.store(true, Ordering::Release);
}

pub struct Tracker {
    tracks: HashMap<ThreadId, BTreeMap<String, Vec<u128>>>,
    instant: Instant,
}

impl Tracker {
    #[inline]
    fn instance() -> &'static mut Self {
        static mut INSTANCE: Lazy<Tracker> = Lazy::new(|| Tracker {
            tracks: HashMap::new(),
            instant: Instant::now(),
        });
        unsafe { &mut INSTANCE }
    }

    #[inline]
    pub fn start<T: ToString>(name: T) -> TrackWrap {
        TrackWrap {
            name: Some(name.to_string()),
            instant: Instant::now(),
            stoped: false,
        }
    }

    pub(crate) fn output_file() -> std::io::Result<()> {
        if !TRACKED.load(Ordering::Relaxed) {
            return Ok(());
        }

        let mut file = File::create(format!("{}_{}.log", TRACK_FILE_NAME, std::process::id()))?;

        let tracker = Self::instance();
        let total_runtime = tracker.instant.elapsed().as_millis() as f64 / 1000.;
        file.write_all(format!("Application total runtime: {}s.\r\n\r\n", total_runtime).as_bytes())?;

        for (id, map) in tracker.tracks.iter_mut() {
            file.write_all(format!("[{:?}]\r\n", id).as_bytes())?;

            for (name, rec) in map.iter_mut() {
                rec.sort();
                let (mut sum, mut max, mut min) = (0., f64::MIN, f64::MAX);

                rec.iter().for_each(|&spend| {
                    let spend = spend as f64 / 1000.;

                    sum += spend;
                    if spend > max {
                        max = spend;
                    }
                    if spend < min {
                        min = spend;
                    }
                });

                let average = sum / rec.len() as f64;
                let median = if rec.len() % 2 == 0 {
                    (rec[rec.len() / 2 - 1] as f64 + rec[rec.len() / 2] as f64) / 2.0 / 1000.
                } else {
                    rec[rec.len() / 2] as f64 / 1000.
                };
                let percentile_95 =
                    rec[(0.95 * (rec.len() - 1) as f64).round() as usize] as f64 / 1000.;
                let percentile_99 =
                    rec[(0.99 * (rec.len() - 1) as f64).round() as usize] as f64 / 1000.;

                file.write_all(format!("{}:\r\n", name).as_bytes())?;
                file.write_all(format!("median time spend         = {}ms\r\n", median).as_bytes())?;
                file.write_all(format!("average time spend        = {:.3}ms\r\n", average).as_bytes())?;
                file.write_all(format!("maximum time spend        = {}ms\r\n", max).as_bytes())?;
                file.write_all(format!("minimum time spend        = {}ms\r\n", min).as_bytes())?;
                file.write_all(
                    format!("percentile 95% time spend = {}ms\r\n", percentile_95).as_bytes(),
                )?;
                file.write_all(
                    format!("percentile 99% time spend = {}ms\r\n", percentile_99).as_bytes(),
                )?;
                file.write_all(
                    format!(
                        "total track count: {}, total time spend {:.3}s, execution proportion: {:.3}%.\r\n\r\n",
                        rec.len(),
                        sum / 1000.,
                        (sum / 1000. / total_runtime) * 100.
                    )
                    .as_bytes(),
                )?;
            }
        }

        Ok(())
    }
}

pub struct TrackWrap {
    name: Option<String>,
    instant: Instant,
    stoped: bool,
}

impl TrackWrap {
    pub fn stop(&mut self) {
        if self.stoped {
            return;
        }
        self.stoped = true;

        if !TRACKED.load(Ordering::Relaxed) {
            return;
        }

        Tracker::instance()
            .tracks
            .entry(std::thread::current().id())
            .or_default()
            .entry(self.name.take().unwrap())
            .or_default()
            .push(self.instant.elapsed().as_micros());
    }
}

impl Drop for TrackWrap {
    fn drop(&mut self) {
        self.stop()
    }
}
