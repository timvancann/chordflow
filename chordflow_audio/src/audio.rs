use std::{fs::File, io::Cursor, path::PathBuf, sync::{mpsc, Arc}, thread, time::Duration};

use chordflow_music_theory::chord::Chord;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

const SAMPLE_RATE: usize = 44100;

pub struct Audio {
    _stream: OutputStream,
    pub synth: Synthesizer,
    pub sink: Sink,
}

pub fn play_chord_with_ticks(
    synth: &mut Synthesizer,
    notes: &[u32],
    chord_duration_ms: u64,
    ticks_per_bar: usize,
) -> Vec<f32> {
    let sample_count = (chord_duration_ms as usize * SAMPLE_RATE) / 1000;
    let mut left = vec![0.0; sample_count];
    let mut right = vec![0.0; sample_count];

    // Play chord
    for note in notes {
        synth.note_on(0, *note as i32, 100);
    }

    synth.render(&mut left, &mut right);

    // Interleave
    let mut buffer = vec![0.0; sample_count * 2];
    for i in 0..sample_count {
        buffer[i * 2] = left[i];
        buffer[i * 2 + 1] = right[i];
    }

    // Play metronome ticks (woodblock sound) every quarter note
    let tick_interval = chord_duration_ms / ticks_per_bar as u64;
    for i in 0..ticks_per_bar {
        let tick_time = i as u64 * tick_interval;
        play_tick(synth, tick_time, &mut buffer);
    }

    // Turn off chord
    for note in notes {
        synth.note_off(0, *note as i32);
    }

    buffer
}

pub fn play_tick(synth: &mut Synthesizer, tick_time: u64, buffer: &mut [f32]) {
    let tick_note = 76; // High Woodblock in General MIDI
    let velocity = 120;

    synth.note_on(9, tick_note, velocity); // Channel 9 = Percussion
    
    let sample_count = SAMPLE_RATE / 10; // ~100ms
    let mut left = vec![0.0; sample_count];
    let mut right = vec![0.0; sample_count];

    synth.render(&mut left, &mut right);
    synth.note_off(9, tick_note);

    // Mix tick buffer into the main buffer at the correct time
    let start_sample_idx = (tick_time as usize * SAMPLE_RATE * 2) / 1000;
    
    for i in 0..sample_count {
        let buf_idx = start_sample_idx + i * 2;
        if buf_idx + 1 < buffer.len() {
            buffer[buf_idx] += left[i];
            buffer[buf_idx + 1] += right[i];
        }
    }
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
        sink.play();

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

pub fn create_synth(soundfont_path: Option<PathBuf>) -> Synthesizer {
    let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    
    let sound_font = if let Some(path) = soundfont_path {
        let mut file = File::open(path).expect("Failed to open SoundFont file");
        SoundFont::new(&mut file).expect("Failed to load SoundFont")
    } else {
        let soundfont_bytes = include_bytes!("../assets/TimGM6mb.sf2");
        let mut cursor = Cursor::new(soundfont_bytes);
        SoundFont::new(&mut cursor).expect("Failed to load embedded SoundFont")
    };

    let sound_font = Arc::new(sound_font);
    Synthesizer::new(&sound_font, &settings).expect("Failed to create synthesizer")
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