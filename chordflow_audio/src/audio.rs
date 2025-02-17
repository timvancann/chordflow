use std::{env, fs::File, io::Write, path::PathBuf};

use chordflow_music_theory::chord::Chord;
use fluidlite::{Settings, Synth};
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

const SAMPLE_RATE: usize = 44100;

pub struct Audio {
    pub synth: Synth,
    pub sink: Sink,
}

pub fn play_chord_with_ticks(
    synth: &mut Synth,
    notes: &[u32],
    chord_duration_ms: u64,
    num_beats: usize,
) -> Vec<f32> {
    let mut buffer = vec![0.0; chord_duration_ms as usize * SAMPLE_RATE * 2 / 1000];

    // Play chord
    for note in notes {
        synth.note_on(0, *note, 100).unwrap();
    }

    synth.write(buffer.as_mut_slice()).unwrap();

    // Play metronome ticks (woodblock sound) every quarter note
    let tick_interval = chord_duration_ms / num_beats as u64;
    for i in 0..num_beats {
        let tick_time = i as u64 * tick_interval;
        play_tick(synth, tick_time, &mut buffer);
    }

    // Turn off chord
    for note in notes {
        synth.note_off(0, *note).unwrap();
    }

    buffer
}

pub fn play_tick(synth: &mut Synth, tick_time: u64, buffer: &mut [f32]) {
    let tick_note = 76; // High Woodblock in General MIDI
    let velocity = 120;

    synth.note_on(9, tick_note, velocity).unwrap(); // Channel 9 = Percussion
    let mut tick_buffer = vec![0.0; SAMPLE_RATE * 2 / 10]; // Small buffer for the tick (~100ms)

    synth.write(tick_buffer.as_mut_slice()).unwrap();
    synth.note_off(9, tick_note).unwrap();

    // Mix tick buffer into the main buffer at the correct time
    let start_sample = (tick_time as usize * SAMPLE_RATE * 2 / 1000).min(buffer.len());
    (0..tick_buffer.len()).for_each(|i| {
        let idx = start_sample + i;
        if idx < buffer.len() {
            buffer[idx] += tick_buffer[i];
        }
    });
}

pub fn setup_audio(soundfont_path: Option<PathBuf>) -> Audio {
    Audio {
        synth: create_synth(soundfont_path),
        sink: create_audio_sink(),
    }
}

pub fn create_synth(soundfont_path: Option<PathBuf>) -> fluidlite::Synth {
    let settings = Settings::new().unwrap();

    let synth = Synth::new(settings).expect("Failed to create synthesizer");
    synth
        .sfload(soundfont_path.unwrap_or(extract_soundfont()), true)
        .unwrap();
    synth
}

fn extract_soundfont() -> PathBuf {
    let file_path = "foo.log";
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

    let mut path = env::temp_dir();
    path.push("guitar_practice_soundfont.sf2"); // Use a fixed filename
    debug!("{:?}", path);

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

pub fn create_audio_sink() -> rodio::Sink {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create audio output stream");
    Sink::try_new(&stream_handle).expect("Failed to create Rodio sink")
}

pub fn note_to_midi(semitones_from_c: i32) -> u32 {
    ((semitones_from_c % 12) + 60) as u32
}

pub fn chord_to_midi(chord: Chord) -> Vec<u32> {
    chord
        .to_c_based_semitones()
        .into_iter()
        .map(note_to_midi)
        .collect()
}

pub fn play(synth: &mut Synth, sink: &Sink, chord: Chord, duration: u64, num_beats: usize) {
    let notes = chord_to_midi(chord);
    let buffer = play_chord_with_ticks(synth, &notes, duration, num_beats);
    let source = SamplesBuffer::new(2, SAMPLE_RATE as u32, buffer);
    sink.append(source);
}
