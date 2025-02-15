use std::{
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use audio::play;
use clap::Parser;
use fluidlite::{Settings, Synth};
use keymap::handle_keys;
use metronome::Metronome;
use mode::Mode;
use music::{note::Note, quality::Quality};
use practice_state::PracticState;
use progression::Progression;
use rodio::{OutputStream, Sink};
use strum::{AsRefStr, EnumCount, FromRepr, IntoEnumIterator};

mod audio;
mod keymap;
mod metronome;
mod mode;
mod music;
mod practice_state;
mod progression;
mod timer;
mod ui;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use strum::Display;
use strum_macros::EnumIter;
use ui::render_ui;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(
        long,
        value_name = "INT",
        help = "BPM (Beats per minute)",
        default_value_t = 100
    )]
    pub bpm: usize,

    #[arg(
        short,
        long,
        value_name = "INT",
        help = "Number of bars per chord",
        default_value_t = 2
    )]
    pub bars_per_chord: usize,

    #[arg(
        short,
        long,
        value_name = "INT",
        help = "Number of beats per bar",
        default_value_t = 4
    )]
    pub ticks_per_bar: usize,

    #[arg(short, long, help = "Soundfont file path")]
    pub soundfont: Option<PathBuf>,
}

fn extract_soundfont() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("guitar_practice_soundfont.sf2"); // Use a fixed filename

    if !path.exists() {
        // Load SoundFont bytes
        let soundfont_bytes = include_bytes!("../assets/TimGM6mb.sf2");

        // Create and write file
        let mut file = File::create(&path).expect("Failed to create temp SoundFont file");
        file.write_all(soundfont_bytes)
            .expect("Failed to write SoundFont file");
    }

    path
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let settings = Settings::new().unwrap();

    let synth = Synth::new(settings).expect("Failed to create synthesizer");
    synth
        .sfload(cli.soundfont.unwrap_or(extract_soundfont()), true)
        .unwrap();

    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create audio output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create Rodio sink");

    let mut terminal = ratatui::init();

    let mut app = App::new(synth, sink);

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

#[derive(Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr)]
enum ModeOption {
    Fourths,
    Diatonic,
    Random,
    Custom,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr)]
enum DiatonicOption {
    Incemental,
    Random,
}

struct App {
    exit: bool,
    selected_tab: AppTab,
    selected_mode: ModeOption,

    fourths_selected_quality: Quality,
    random_selected_qualities: Vec<Quality>,
    random_qualities_cursor: Quality,
    custom_input_buffer: String,
    custom_parsed_progression: Option<Progression>,
    diatonic_selected_option: DiatonicOption,
    diatonic_selected_root: Note,

    metronome: Metronome,
    practice_state: PracticState,

    synth: Synth,
    sink: Sink,
}

impl App {
    fn new(synth: Synth, sink: Sink) -> Self {
        Self {
            exit: false,
            selected_tab: AppTab::Mode,
            selected_mode: ModeOption::Fourths,
            fourths_selected_quality: Quality::Major,
            random_selected_qualities: Quality::iter().collect(),
            random_qualities_cursor: Quality::Major,
            custom_input_buffer: String::new(),
            custom_parsed_progression: None,
            diatonic_selected_option: DiatonicOption::Incemental,
            diatonic_selected_root: Note::new(music::note::NoteLetter::C, 0),
            metronome: Metronome::new(100, 2, 4, Instant::now),
            practice_state: PracticState::default(),
            synth,
            sink,
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
        self.metronome.start();
        self.selected_tab = AppTab::Playback;
        play(
            &mut self.synth,
            &self.sink,
            self.practice_state.current_chord,
            self.metronome.duration_per_bar,
            self.metronome.num_beats,
        );
        while !self.exit {
            terminal.draw(|f| render_ui(f, self))?;
            self.update();
            self.handle_events()?;
        }
        Ok(())
    }
    fn update(&mut self) {
        self.metronome.tick();
        if self.metronome.has_cycle_ended() {
            if let Mode::Custom(Some(p)) = &self.practice_state.mode {
                self.metronome.num_bars =
                    p.chords[self.practice_state.next_progression_chord_idx].bars;
            }
            self.practice_state.next_chord();
            self.metronome.reset();
        }
        if self.metronome.has_bar_ended() {
            play(
                &mut self.synth,
                &self.sink,
                self.practice_state.current_chord,
                self.metronome.duration_per_bar,
                self.metronome.num_beats,
            );
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                handle_keys(key, self);
            }
        }
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}
