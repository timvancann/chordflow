use crate::{audio::settings::AUDIO_SETTINGS, AudioCommand, AudioEvent, AUDIO_CMD, AUDIO_EVT};
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::{
    fs::File,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU64, Ordering},
        Arc,
    },
};

/// Find the soundfont file, checking both development and bundled app locations
fn find_soundfont_path() -> Result<PathBuf> {
    const SOUNDFONT_NAME: &str = "TimGM6mb.sf2";

    // Try development path first
    let dev_path = PathBuf::from("assets").join(SOUNDFONT_NAME);
    if dev_path.exists() {
        return Ok(dev_path);
    }

    // Try macOS bundle path: executable is in .app/Contents/MacOS/
    // Resources are in .app/Contents/Resources/
    if let Ok(exe_path) = std::env::current_exe() {
        // Go from MacOS/ to Resources/
        if let Some(macos_dir) = exe_path.parent() {
            let resources_dir = macos_dir.parent().map(|p| p.join("Resources"));
            if let Some(resources) = resources_dir {
                // Check directly in Resources
                let bundle_path = resources.join(SOUNDFONT_NAME);
                if bundle_path.exists() {
                    return Ok(bundle_path);
                }
                // Check in Resources/assets
                let bundle_assets_path = resources.join("assets").join(SOUNDFONT_NAME);
                if bundle_assets_path.exists() {
                    return Ok(bundle_assets_path);
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not find soundfont file '{}' in any expected location",
        SOUNDFONT_NAME
    ))
}

pub fn init_stream() -> Result<Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No default output device"))?;
    let channels = device.default_output_config()?.channels() as usize;
    // Start from the default output configuration so we match the device's expectations
    let config = device.default_output_config()?.config();
    let sample_rate = config.sample_rate.0;

    let bpm: Arc<AtomicU16> = Arc::new(AtomicU16::new(120));
    let is_playing: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let sample_counter = Arc::new(AtomicU64::new(0));
    let next_click_sample = Arc::new(AtomicU64::new(0));
    let chord: Arc<parking_lot::Mutex<Option<Vec<u8>>>> = Arc::new(parking_lot::Mutex::new(None));
    let subdivisions_per_beat: Arc<AtomicU8> = Arc::new(AtomicU8::new(1));
    let current_subdivision: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
    let ticks_per_bar: Arc<AtomicU8> = Arc::new(AtomicU8::new(4));
    let current_beat_in_bar: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));

    // Load and verify soundfont
    let sf2_path = find_soundfont_path()?;
    let mut sf2 = File::open(&sf2_path)
        .map_err(|e| anyhow::anyhow!("Failed to open soundfont file at {:?}: {}", sf2_path, e))?;
    let sound_font = Arc::new(
        SoundFont::new(&mut sf2)
            .map_err(|e| anyhow::anyhow!("Failed to parse soundfont: {}", e))?,
    );

    // Create the synthesizer.
    let settings = SynthesizerSettings::new(sample_rate as i32);
    let synthesizer = Arc::new(parking_lot::Mutex::new(
        Synthesizer::new(&sound_font, &settings)
            .map_err(|e| anyhow::anyhow!("Failed to create synthesizer: {}", e))?,
    ));

    // Clone atomics for the command handler thread
    let bpm_cmd = bpm.clone();
    let is_playing_cmd = is_playing.clone();
    let next_click_sample_cmd = next_click_sample.clone();
    let sample_counter_cmd = sample_counter.clone();
    let chord_cmd = chord.clone();
    let subdivisions_per_beat_cmd = subdivisions_per_beat.clone();
    let current_subdivision_cmd = current_subdivision.clone();
    let current_beat_in_bar_cmd = current_beat_in_bar.clone();

    // Spawn a dedicated thread to handle audio commands
    std::thread::spawn(move || {
        loop {
            while let Ok(cmd) = AUDIO_CMD.1.try_recv() {
                match cmd {
                    AudioCommand::Start => {
                        // Reset all counters to start of bar
                        current_subdivision_cmd.store(0, Ordering::Relaxed);
                        current_beat_in_bar_cmd.store(0, Ordering::Relaxed);
                        // Schedule first tick immediately
                        let current = sample_counter_cmd.load(Ordering::Relaxed);
                        next_click_sample_cmd.store(current, Ordering::Relaxed);
                        // Start playing
                        is_playing_cmd.store(true, Ordering::Relaxed);
                    }
                    AudioCommand::Stop => {
                        is_playing_cmd.store(false, Ordering::Relaxed);
                        // Reset counters so next start is clean
                        current_subdivision_cmd.store(0, Ordering::Relaxed);
                        current_beat_in_bar_cmd.store(0, Ordering::Relaxed);
                    }
                    AudioCommand::Restart => {
                        // Reset all counters to start of bar
                        current_subdivision_cmd.store(0, Ordering::Relaxed);
                        current_beat_in_bar_cmd.store(0, Ordering::Relaxed);
                        // Schedule next tick immediately if playing
                        if is_playing_cmd.load(Ordering::Relaxed) {
                            let current = sample_counter_cmd.load(Ordering::Relaxed);
                            next_click_sample_cmd.store(current, Ordering::Relaxed);
                        }
                    }
                    AudioCommand::SetBPM(new_bpm) => {
                        bpm_cmd.store(new_bpm, Ordering::Relaxed);
                    }
                    AudioCommand::SetSubdivision(subdivs) => {
                        subdivisions_per_beat_cmd.store(subdivs, Ordering::Relaxed);
                        // Reset counters to start of bar when changing subdivision
                        current_subdivision_cmd.store(0, Ordering::Relaxed);
                        current_beat_in_bar_cmd.store(0, Ordering::Relaxed);
                        // Reschedule next tick immediately if playing
                        if is_playing_cmd.load(Ordering::Relaxed) {
                            let current = sample_counter_cmd.load(Ordering::Relaxed);
                            next_click_sample_cmd.store(current, Ordering::Relaxed);
                        }
                    }
                    AudioCommand::SetChord(midi_notes) => {
                        if let Some(notes) = midi_notes {
                            *chord_cmd.lock() = Some(notes);
                        } else {
                            *chord_cmd.lock() = None;
                        }
                    }
                    _ => {}
                }
            }
            // Small sleep to avoid busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });

    // Percussion channel (MIDI channel 10 = index 9)
    const PERCUSSION_CHANNEL: i32 = 9;

    // Different click sounds for different beat types
    // Using woodblock and sidestick sounds from General MIDI percussion
    const CLICK_ACCENT: i32 = 76;     // Hi wood block - for downbeat (first beat of bar)
    const CLICK_NORMAL: i32 = 77;     // Low wood block - for regular beats
    const CLICK_SUBDIVISION: i32 = 37; // Side stick - for subdivisions

    // Velocities for different beat types
    const VELOCITY_ACCENT: i32 = 120;
    const VELOCITY_NORMAL: i32 = 100;
    const VELOCITY_SUBDIVISION: i32 = 70;

    // Chord configuration
    const CHORD_CHANNEL: i32 = 0; // Use channel 0 for melodic instruments
    const CHORD_VELOCITY: i32 = 80;

    let synth_clone = synthesizer.clone();
    let chord_clone = chord.clone();
    let stream = device.build_output_stream(
        &config,
        move |buffer: &mut [f32], _: &cpal::OutputCallbackInfo| {
            buffer.fill(0.0);

            // Total number of samples that have been generated so far
            let current_sample = sample_counter.load(Ordering::Relaxed);

            if !is_playing.load(Ordering::Relaxed) {
                return;
            }

            let current_bpm = bpm.load(Ordering::Relaxed);
            let subdivs = subdivisions_per_beat.load(Ordering::Relaxed) as u64;

            // How many frames (multi-channel sample groups) we must fill in this callback
            let frames = buffer.len() / channels;

            // Calculate samples per beat and per subdivision
            let samples_per_beat = (sample_rate as f64 * 60.0 / current_bpm as f64) as u64;
            let samples_per_subdivision = samples_per_beat / subdivs;

            for frame in 0..frames {
                let next_click = next_click_sample.load(Ordering::Relaxed);
                let frame_sample = current_sample + frame as u64;

                if frame_sample >= next_click {
                    let left_over_frames = frame_sample - next_click;
                    let mut synth = synth_clone.lock();

                    // Get current subdivision within the beat (0 = main beat)
                    let curr_subdiv = current_subdivision.load(Ordering::Relaxed);
                    let curr_beat = current_beat_in_bar.load(Ordering::Relaxed);

                    // Determine which sound to play and apply volume settings
                    let (note, base_velocity, volume_multiplier) = if curr_subdiv == 0 {
                        // This is a main beat
                        if curr_beat == 0 {
                            // First beat of bar - accent
                            (
                                CLICK_ACCENT,
                                VELOCITY_ACCENT,
                                AUDIO_SETTINGS.get_metronome_accent_volume(),
                            )
                        } else {
                            // Regular beat
                            (
                                CLICK_NORMAL,
                                VELOCITY_NORMAL,
                                AUDIO_SETTINGS.get_metronome_beat_volume(),
                            )
                        }
                    } else {
                        // This is a subdivision
                        (
                            CLICK_SUBDIVISION,
                            VELOCITY_SUBDIVISION,
                            AUDIO_SETTINGS.get_metronome_subdivision_volume(),
                        )
                    };

                    // Apply volume and clamp to valid MIDI velocity range (0-127)
                    let velocity = ((base_velocity as f32) * volume_multiplier).clamp(0.0, 127.0) as i32;

                    // Trigger click sound
                    synth.note_on(PERCUSSION_CHANNEL, note, velocity);

                    // Only send Tick event and play chord on main beats (not subdivisions)
                    if curr_subdiv == 0 {
                        let _ = AUDIO_EVT.0.try_send(AudioEvent::Tick);

                        // Play chord if one is set
                        if let Some(ref midi_notes) = *chord_clone.lock() {
                            let chord_volume = AUDIO_SETTINGS.get_chord_volume();
                            let chord_velocity =
                                ((CHORD_VELOCITY as f32) * chord_volume).clamp(0.0, 127.0) as i32;
                            for &note in midi_notes {
                                synth.note_on(CHORD_CHANNEL, note as i32, chord_velocity);
                            }
                        }
                    }

                    // Advance subdivision counter
                    let next_subdiv = (curr_subdiv + 1) % subdivs as u8;
                    current_subdivision.store(next_subdiv, Ordering::Relaxed);

                    // If we've completed all subdivisions, advance beat counter
                    if next_subdiv == 0 {
                        let next_beat = (curr_beat + 1) % ticks_per_bar.load(Ordering::Relaxed);
                        current_beat_in_bar.store(next_beat, Ordering::Relaxed);
                    }

                    // Schedule next click (subdivision or main beat)
                    next_click_sample.store(
                        next_click + samples_per_subdivision - left_over_frames,
                        Ordering::Relaxed,
                    );
                }
            }

            // Render synthesizer output
            let mut synth = synth_clone.lock();

            if channels == 2 {
                // Stereo output
                let mut left = vec![0.0; frames];
                let mut right = vec![0.0; frames];

                synth.render(&mut left[..], &mut right[..]);

                // Mix synthesizer output into buffer
                for (i, frame) in buffer.chunks_exact_mut(2).enumerate() {
                    frame[0] += left[i];
                    frame[1] += right[i];
                }
            } else {
                // Mono output - need separate buffers for render
                let mut left = vec![0.0; frames];
                let mut right = vec![0.0; frames];
                synth.render(&mut left[..], &mut right[..]);

                // Mix down to mono
                for (i, sample) in buffer.iter_mut().enumerate() {
                    *sample += (left[i / channels] + right[i / channels]) * 0.5;
                }
            }

            sample_counter.fetch_add(frames as u64, Ordering::Relaxed);
        },
        move |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    stream.play()?;
    Ok(stream)
}
