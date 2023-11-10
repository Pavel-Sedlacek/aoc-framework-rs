use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    lap_start: Instant,
    laps: Vec<Duration>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
            lap_start: Instant::now(),
            laps: vec![],
        }
    }

    pub fn lap(&mut self) {
        self.laps.push(Instant::now() - self.lap_start);
        self.lap_start = Instant::now();
    }

    pub fn get_lap(&self, lap: usize) -> Option<&Duration> {
        self.laps.get(lap)
    }

    pub fn get(&self) -> Duration {
        Instant::now() - self.start
    }

    pub fn get_laps(&self) -> &Vec<Duration> {
        &self.laps
    }
}
