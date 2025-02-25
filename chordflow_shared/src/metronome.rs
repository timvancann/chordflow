use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub struct Metronome {
    pub num_bars: usize,
    pub num_ticks: usize,
    pub bpm: usize,

    pub timer_source: Box<dyn Fn() -> Instant + Send>,
    pub last_tick_time: Instant,

    pub current_tick: usize,
    pub current_bar: usize,
    pub is_running: bool,
}

impl Metronome {
    pub fn new(
        bpm: usize,
        num_bars: usize,
        num_beats: usize,
        timer_source: impl Fn() -> Instant + 'static + Send,
    ) -> Self {
        Metronome {
            last_tick_time: timer_source(),
            timer_source: Box::new(timer_source),
            bpm,
            current_bar: 0,
            current_tick: 0,
            num_bars,
            num_ticks: num_beats,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.last_tick_time = (self.timer_source)();
    }

    pub fn has_bar_ended(&self) -> bool {
        self.current_tick % self.num_ticks == 0
    }

    pub fn has_cycle_ended(&self) -> bool {
        self.has_bar_ended() && self.current_bar % self.num_bars == 0
    }

    pub fn tick(&mut self) -> usize {
        self.current_tick += 1;
        if self.current_tick % self.num_ticks == 0 {
            self.current_bar = (self.current_bar + 1) % self.num_bars;
            self.current_tick = 0;
        }
        self.current_tick
    }

    pub fn increase_bpm(&mut self, delta: usize) {
        self.bpm = self.bpm.saturating_add(delta)
    }
    pub fn decrease_bpm(&mut self, delta: usize) {
        self.bpm = self.bpm.saturating_sub(delta)
    }

    pub fn get_tick_duration(&self) -> Duration {
        Duration::from_millis((60_000 / self.bpm) as u64)
    }

    pub fn reset(&mut self) {
        self.current_bar = 0;
        self.current_tick = 0;
        self.reset_timers();
    }

    pub fn reset_timers(&mut self) {
        self.last_tick_time = (self.timer_source)();
    }
}

pub struct NoteDuration {
    pub duration_per_quarter_note: u64,
    pub duration_per_bar: u64,
}

pub fn calculate_duration_per_bar(bpm: usize, ticks_per_bar: usize) -> NoteDuration {
    let duration_per_quarter_note = 60000u64 / bpm as u64;
    let duration_per_bar = duration_per_quarter_note * ticks_per_bar as u64;
    NoteDuration {
        duration_per_quarter_note,
        duration_per_bar,
    }
}

pub enum MetronomeCommand {
    DecreaseBpm(usize),
    IncreaseBpm(usize),
    Reset,
    SetBars(usize),
    Stop,
}

pub enum MetronomeEvent {
    BarComplete(usize),
    CycleComplete,
    Tick(usize),
}

pub fn setup_metronome(
    bpm: usize,
    num_bars: usize,
    num_beats: usize,
    timer_source: impl Fn() -> Instant + 'static + Send + Copy,
) -> (
    mpsc::Sender<MetronomeCommand>,
    mpsc::Receiver<MetronomeEvent>,
) {
    let (tx_command, rx_command) = mpsc::channel();
    let (tx_event, rx_event) = mpsc::channel();

    thread::spawn(move || {
        let mut metronome = Metronome::new(bpm, num_bars, num_beats, timer_source);
        let mut running = true;

        metronome.last_tick_time = timer_source();
        loop {
            while let Ok(command) = rx_command.try_recv() {
                match command {
                    MetronomeCommand::SetBars(n) => metronome.num_bars = n,
                    MetronomeCommand::Reset => metronome.reset(),
                    MetronomeCommand::IncreaseBpm(delta) => metronome.increase_bpm(delta),
                    MetronomeCommand::DecreaseBpm(delta) => metronome.decrease_bpm(delta),
                    MetronomeCommand::Stop => {
                        running = false;
                        break;
                    }
                }
            }

            if !running {
                break;
            }

            let tick_duration = metronome.get_tick_duration();
            let elapsed = (timer_source)().duration_since(metronome.last_tick_time);

            if elapsed >= tick_duration {
                let current_tick = metronome.tick();
                let _ = tx_event.send(MetronomeEvent::Tick(current_tick));
                if metronome.has_cycle_ended() {
                    let _ = tx_event.send(MetronomeEvent::CycleComplete);
                }
                if metronome.has_bar_ended() {
                    let _ = tx_event.send(MetronomeEvent::BarComplete(metronome.current_bar));
                }

                // Reset the timer
                metronome.last_tick_time = (timer_source)();
            } else {
                // Sleep for a short duration to avoid busy waiting
                // But keep it short to maintain responsiveness
                let sleep_duration = std::cmp::min(
                    tick_duration.saturating_sub(elapsed),
                    Duration::from_millis(1),
                );
                thread::sleep(sleep_duration);
            }
        }
    });

    (tx_command, rx_event)
}
