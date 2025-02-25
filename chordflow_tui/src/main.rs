use chordflow_shared::cli::parse_cli;
use chordflow_shared::metronome::{
    calculate_duration_per_bar, setup_metronome, MetronomeCommand, MetronomeEvent,
};
use chordflow_shared::practice_state::ConfigState;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::io;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

use chordflow_audio::audio::{setup_audio, AudioCommand};
use chordflow_music_theory::quality::Quality;
use chordflow_shared::{mode::Mode, practice_state::PracticState, DiatonicOption, ModeOption};
use strum::{AsRefStr, EnumCount, FromRepr, IntoEnumIterator};

mod keymap;
mod ui;

use crossterm::event::{self, Event};
use keymap::handle_keys;
use ratatui::DefaultTerminal;
use strum::Display;
use strum_macros::EnumIter;
use ui::render_ui;

#[cfg(debug_assertions)]
fn setup_logging() {
    let file_path = "tui.log";
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    let _handle = log4rs::init_config(config);
}

fn main() -> io::Result<()> {
    let cli = parse_cli();

    #[cfg(debug_assertions)]
    setup_logging();

    let mut terminal = ratatui::init();
    let tx_audio = setup_audio(cli.soundfont);
    let mut app = App::new(tx_audio, cli.bpm, cli.bars_per_chord, cli.ticks_per_bar);

    app.run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
#[derive(Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr)]
enum AppTab {
    Mode,
    Config,
    Playback,
}

struct App {
    exit: bool,
    selected_tab: AppTab,
    selected_mode: ModeOption,

    bars_per_chord: usize,
    ticks_per_bar: usize,
    bpm: usize,
    current_bar: usize,
    current_tick: usize,

    config_state: ConfigState,

    random_qualities_cursor: Quality,
    custom_input_buffer: String,
    practice_state: PracticState,

    tx_audio: Sender<AudioCommand>,
    tx_metronome: Sender<MetronomeCommand>,
    rx_metronome: Receiver<MetronomeEvent>,
}

impl App {
    fn new(
        tx_audio: Sender<AudioCommand>,
        bpm: usize,
        bars_per_chord: usize,
        ticks_per_bar: usize,
    ) -> Self {
        let (tx_metronome, rx_metronome) =
            setup_metronome(bpm, bars_per_chord, ticks_per_bar, Instant::now);
        Self {
            exit: false,
            selected_tab: AppTab::Playback,
            selected_mode: ModeOption::Fourths,
            config_state: ConfigState::default(),
            random_qualities_cursor: Quality::Major,
            custom_input_buffer: String::new(),
            practice_state: PracticState::default(),
            bars_per_chord,
            ticks_per_bar,
            current_bar: 0,
            current_tick: 0,
            bpm,
            tx_audio,
            tx_metronome,
            rx_metronome,
        }
    }

    fn next_item<T>(&mut self, current_item: T) -> usize
    where
        T: EnumCount + IntoEnumIterator + PartialEq,
    {
        let current_position = T::iter().position(|t| t == current_item).unwrap();
        let next_position: usize = (current_position + 1) % T::COUNT;
        next_position
    }

    fn prev_item<T>(&mut self, current_item: T) -> usize
    where
        T: EnumCount + IntoEnumIterator + PartialEq,
    {
        let current_position = T::iter().position(|t| t == current_item).unwrap();
        let prev_position: usize = if current_position == 0 {
            T::COUNT - 1
        } else {
            current_position - 1
        };
        prev_position
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let _ = self.tx_audio.send(AudioCommand::PlayChord((
            self.practice_state.current_chord,
            calculate_duration_per_bar(self.bpm, self.ticks_per_bar).duration_per_bar,
            self.ticks_per_bar,
        )));

        while !self.exit {
            terminal.draw(|f| render_ui(f, self))?;
            self.handle_events()?;
            self.update();
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                handle_keys(key, self);
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        while let Ok(event) = self.rx_metronome.try_recv() {
            match event {
                MetronomeEvent::CycleComplete => {
                    if let Mode::Custom(Some(p)) = &self.practice_state.mode {
                        self.bars_per_chord =
                            p.chords[self.practice_state.next_progression_chord_idx].bars;
                    }
                    let _ = self
                        .tx_metronome
                        .send(MetronomeCommand::SetBars(self.bars_per_chord));
                    let _ = self.tx_metronome.send(MetronomeCommand::Reset);
                    self.practice_state.next_chord();
                    self.current_bar = 0;
                    self.current_tick = 0;
                }
                MetronomeEvent::BarComplete(b) => {
                    let _ = self.tx_audio.send(AudioCommand::PlayChord((
                        self.practice_state.current_chord,
                        calculate_duration_per_bar(self.bpm, self.ticks_per_bar).duration_per_bar,
                        self.ticks_per_bar,
                    )));
                    self.current_bar = b;
                    self.current_tick = 0;
                }
                MetronomeEvent::Tick(t) => self.current_tick = t,
            };
        }
    }
}

fn sync_metronome_bars(app: &mut App) {
    if let Mode::Custom(Some(p)) = &app.practice_state.mode {
        app.bars_per_chord = p.chords[app.practice_state.next_progression_chord_idx].bars;
    }
    let _ = app
        .tx_metronome
        .send(MetronomeCommand::SetBars(app.bars_per_chord));
}
