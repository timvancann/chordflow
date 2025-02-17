use std::{
    io::{self},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use chordflow_audio::audio::{self, create_audio_sink, create_synth, play, setup_audio, Audio};
use chordflow_music_theory::{
    note::{Note, NoteLetter},
    quality::Quality,
};
use chordflow_shared::{
    metronome::Metronome, mode::Mode, practice_state::PracticState, progression::Progression,
    DiatonicOption, ModeOption,
};
use clap::Parser;
use strum::{AsRefStr, EnumCount, FromRepr, IntoEnumIterator};

mod keymap;
mod ui;

use crossterm::event::{self, Event};
use keymap::handle_keys;
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

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut terminal = ratatui::init();

    let mut app = App::new(setup_audio(cli.soundfont));

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

    fourths_selected_quality: Quality,
    random_selected_qualities: Vec<Quality>,
    random_qualities_cursor: Quality,
    custom_input_buffer: String,
    custom_parsed_progression: Option<Progression>,
    diatonic_selected_option: DiatonicOption,
    diatonic_selected_root: Note,

    metronome: Metronome,
    practice_state: PracticState,

    audio: Audio,
}

impl App {
    fn new(audio: Audio) -> Self {
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
            diatonic_selected_root: Note::new(NoteLetter::C, 0),
            metronome: Metronome::new(100, 2, 4, Instant::now),
            practice_state: PracticState::default(),
            audio,
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
            &mut self.audio.synth,
            &self.audio.sink,
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
                &mut self.audio.synth,
                &self.audio.sink,
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
