use crate::AudioCommand;
use crate::AudioEvent;
use crate::AUDIO_CMD;
use crate::AUDIO_EVT;
use anyhow::Result;
use cpal::traits::HostTrait;
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

pub fn init_stream() -> Result<Stream> {
    println!("Initializing audio stream...");
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No default output device"))?;
    let channels = device.default_output_config()?.channels() as usize;
    // Start from the default output configuration so we match the device's expectations
    let config = device.default_output_config()?.config();
    let sample_rate = config.sample_rate.0;
    println!(
        "Audio device configured: {} channels, {} Hz",
        channels, sample_rate
    );

    let bpm: Arc<AtomicU16> = Arc::new(AtomicU16::new(120));
    let is_playing: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let sample_counter = Arc::new(AtomicU64::new(0));
    let next_click_sample = Arc::new(AtomicU64::new(0));

    // Load and verify soundfont
    println!("Loading soundfont...");
    let mut sf2 = File::open("assets/TimGM6mb.sf2")
        .map_err(|e| anyhow::anyhow!("Failed to open soundfont file: {}", e))?;
    println!("Soundfont file opened, parsing...");
    let sound_font = Arc::new(
        SoundFont::new(&mut sf2)
            .map_err(|e| anyhow::anyhow!("Failed to parse soundfont: {}", e))?,
    );

    println!(
        "Soundfont loaded successfully with {} presets",
        sound_font.get_presets().len()
    );

    // Create the synthesizer.
    println!("Creating synthesizer...");
    let settings = SynthesizerSettings::new(sample_rate as i32);
    let synthesizer = Arc::new(parking_lot::Mutex::new(
        Synthesizer::new(&sound_font, &settings)
            .map_err(|e| anyhow::anyhow!("Failed to create synthesizer: {}", e))?,
    ));
    println!("Synthesizer created successfully");

    // Clone atomics for the command handler thread
    println!("Setting up command handler thread...");
    let bpm_cmd = bpm.clone();
    let is_playing_cmd = is_playing.clone();
    let next_click_sample_cmd = next_click_sample.clone();
    let sample_counter_cmd = sample_counter.clone();

    // Spawn a dedicated thread to handle audio commands
    println!("Spawning command handler thread...");
    std::thread::spawn(move || {
        println!("Command handler thread started");
        loop {
            while let Ok(cmd) = AUDIO_CMD.1.try_recv() {
                match cmd {
                    AudioCommand::Start => {
                        is_playing_cmd.store(true, Ordering::Relaxed);
                        // Schedule first tick immediately using current sample counter
                        let current = sample_counter_cmd.load(Ordering::Relaxed);
                        next_click_sample_cmd.store(current, Ordering::Relaxed);
                    }
                    AudioCommand::Stop => {
                        is_playing_cmd.store(false, Ordering::Relaxed);
                    }
                    AudioCommand::SetBPM(new_bpm) => {
                        bpm_cmd.store(new_bpm, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
            // Small sleep to avoid busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });

    // Woodblock is typically on MIDI channel 10 (percussion), note 76
    const WOODBLOCK_CHANNEL: i32 = 9; // MIDI channels are 0-indexed, channel 10 = index 9
    const WOODBLOCK_NOTE: i32 = 76;
    const WOODBLOCK_VELOCITY: i32 = 100;

    println!("Building output stream...");
    let synth_clone = synthesizer.clone();
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

            // How many frames (multi-channel sample groups) we must fill in this callback
            let frames = buffer.len() / channels;

            // Calculate samples per beat
            let samples_per_beat = (sample_rate as f64 * 60.0 / current_bpm as f64) as u64;

            for frame in 0..frames {
                let next_click = next_click_sample.load(Ordering::Relaxed);
                let frame_sample = current_sample + frame as u64;

                if frame_sample >= next_click {
                    // Trigger woodblock sound
                    let mut synth = synth_clone.lock();
                    synth.note_on(WOODBLOCK_CHANNEL, WOODBLOCK_NOTE, WOODBLOCK_VELOCITY);

                    let _ = AUDIO_EVT.0.try_send(AudioEvent::Tick);

                    // Schedule next tick
                    next_click_sample.store(next_click + samples_per_beat, Ordering::Relaxed);
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

    println!("Stream built, starting playback...");
    stream.play()?;
    println!("Audio stream initialized and playing!");
    Ok(stream)
}
