use std::sync::LazyLock;

use crossbeam_channel::{bounded, Receiver, Sender};
use dioxus::desktop::{
    tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder,
};

use crate::{audio::stream::init_stream, ui::app::App};

mod audio;
mod progression;
mod state;
mod ui;

pub enum AudioCommand {
    Start,
    Stop,
    SetBPM(u16),
    SetBarsPerCycle(u8),
    SetSubdivision(u8, u8),
}

pub enum AudioEvent {
    Tick,
}

pub enum MetronomeEvent {
    BarComplete,
    CycleComplete,
}

pub const INITIAL_BPM: u16 = 100;

pub static AUDIO_CMD: LazyLock<(Sender<AudioCommand>, Receiver<AudioCommand>)> =
    LazyLock::new(|| bounded(128));
pub static AUDIO_EVT: LazyLock<(Sender<AudioEvent>, Receiver<AudioEvent>)> =
    LazyLock::new(|| bounded(64));
pub static METRONOME_EVT: LazyLock<(Sender<MetronomeEvent>, Receiver<MetronomeEvent>)> =
    LazyLock::new(|| bounded(64));

fn main() {
    let window_builder = WindowBuilder::new()
        .with_transparent(false)
        .with_decorations(true)
        .with_focused(true)
        .with_resizable(true)
        .with_title("ChordFlow")
        .with_has_shadow(true)
        .with_movable_by_window_background(true)
        .with_inner_size(LogicalSize {
            height: 910,
            width: 1000,
        })
        .with_always_on_top(false);

    let config = Config::default().with_window(window_builder);

    println!("Initializing audio system...");
    // Initialize and leak the audio stream to keep it alive for the application lifetime
    let stream = init_stream().expect("Failed to initialize audio stream");
    println!("Audio stream created, leaking to keep alive...");
    Box::leak(Box::new(stream));

    println!("Launching Dioxus application...");
    dioxus::LaunchBuilder::new().with_cfg(config).launch(App)
}
