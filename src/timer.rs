#[derive(Debug)]
enum TimerState {
    Running,
    Paused,
}
pub struct Timer {
    pub duration_ms: f64,
    elapsed_ms: f64,
    last_time: std::time::Instant,
    state: TimerState,
    pub ended: bool,
    pub(crate) time_source: Box<dyn Fn() -> std::time::Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new(0.0, Box::new(std::time::Instant::now))
    }
}

impl Timer {
    pub fn new(duration_ms: f64, time_source: impl Fn() -> std::time::Instant + 'static) -> Timer {
        Timer {
            duration_ms,
            elapsed_ms: 0.0,
            last_time: time_source(),
            state: TimerState::Paused,
            ended: false,
            time_source: Box::new(time_source),
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Paused => self.start(),
            TimerState::Running => self.pause(),
        }
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.last_time = self.time_source.as_mut()();
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
    }

    pub fn reset(&mut self) {
        self.elapsed_ms = 0.;
        self.ended = false
    }

    pub fn tick(&mut self) {
        match self.state {
            TimerState::Running => {
                self.ended = false;
                let now = self.time_source.as_mut()();
                let delta = now.duration_since(self.last_time);
                self.elapsed_ms += delta.as_secs_f64() * 1000.0;
                self.last_time = now;
                if self.elapsed_ms >= self.duration_ms {
                    self.ended = true;
                    self.elapsed_ms -= self.duration_ms
                }
            }
            TimerState::Paused => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::timer::Timer;

    #[test]
    fn test_tick() {
        use std::time::{Duration, Instant};

        let fake_time = Instant::now();
        let time_source = move || fake_time;

        let mut timer = Timer::new(1000.0, time_source);

        // Start timer
        timer.start();

        // Simulate time passing
        timer.time_source = Box::new(move || fake_time + Duration::from_millis(500));
        timer.tick();
        assert!(!timer.ended);
        assert!(timer.elapsed_ms >= 500.0);

        timer.time_source = Box::new(move || fake_time + Duration::from_millis(1050));
        timer.tick();
        assert!(timer.ended);
        assert!(timer.elapsed_ms < 100.0)
    }
}
