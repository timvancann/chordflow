use std::{env, fs::File, io::Write, path::PathBuf, sync::mpsc, thread, time::Duration};

use chordflow_music_theory::chord::Chord;
use fluidlite::{Settings, Synth};
use log::debug;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

const SAMPLE_RATE: usize = 44100;

pub struct Audio {
    _stream: OutputStream,
    pub synth: Synth,
    pub sink: Sink,
}

pub fn play_chord_with_ticks(
    synth: &mut Synth,
    notes: &[u32],
    chord_duration_ms: u64,
    ticks_per_bar: usize,
) -> Vec<f32> {
    let mut buffer = vec![0.0; chord_duration_ms as usize * SAMPLE_RATE * 2 / 1000];

    // Play chord
    for note in notes {
        synth.note_on(0, *note, 100).unwrap();
    }

    synth.write(buffer.as_mut_slice()).unwrap();

    // Play metronome ticks (woodblock sound) every quarter note
    let tick_interval = chord_duration_ms / ticks_per_bar as u64;
    for i in 0..ticks_per_bar {
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

#[derive(Clone, PartialEq)]
pub enum AudioCommand {
    PlayChord((Chord, u64, usize)),
    Play,
    Pause,
}

pub fn setup_audio(soundfont_path: Option<PathBuf>) -> mpsc::Sender<AudioCommand> {
    let (tx, rx) = mpsc::channel();
    let mut synth = create_synth(soundfont_path);

    thread::spawn(move || {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Failed to create audio output stream");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create Rodio sink");

        loop {
            while let Ok(command) = rx.try_recv() {
                match command {
                    AudioCommand::PlayChord((chord, duration, ticks_per_bar)) => {
                        sink.stop();
                        let notes = chord_to_midi(chord);
                        let buffer =
                            play_chord_with_ticks(&mut synth, &notes, duration, ticks_per_bar);
                        let source = SamplesBuffer::new(2, SAMPLE_RATE as u32, buffer);
                        sink.append(source);
                        sink.play();
                    }
                    AudioCommand::Pause => {
                        if !sink.is_paused() {
                            sink.pause();
                        }
                    }
                    AudioCommand::Play => {
                        if sink.is_paused() {
                            sink.play();
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    tx
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

pub fn create_audio_sink() -> (rodio::Sink, OutputStream) {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create audio output stream");
    (
        Sink::try_new(&stream_handle).expect("Failed to create Rodio sink"),
        _stream,
    )
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
