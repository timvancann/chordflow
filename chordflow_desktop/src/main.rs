use std::sync::LazyLock;

use crossbeam_channel::{bounded, Receiver, Sender};
#[cfg(target_os = "macos")]
use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

use crate::{audio::stream::init_stream, ui::app::App};

mod audio;
mod components;
mod state;
mod ui;

pub enum AudioCommand {
    Start,
    StartWithCountIn,
    Stop,
    Restart,
    SetBPM(u16),
    SetBarsPerCycle(u8),
    SetSubdivision(u8),
    SetChord(Option<Vec<u8>>),
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
    // Set up logging to file so we can debug bundled app issues
    if let Err(e) = setup_logging() {
        eprintln!("Failed to set up logging: {}", e);
    }

    log::info!("Starting ChordFlow...");
    log::info!("Current working directory: {:?}", std::env::current_dir());
    log::info!("Executable path: {:?}", std::env::current_exe());

    let mut window_builder = WindowBuilder::new()
        .with_transparent(false)
        .with_decorations(true)
        .with_focused(true)
        .with_resizable(true)
        .with_title("ChordFlow");

    #[cfg(target_os = "macos")]
    {
        window_builder = window_builder
            .with_has_shadow(true)
            .with_movable_by_window_background(true);
    }

    window_builder = window_builder
        .with_inner_size(LogicalSize {
            height: 910,
            width: 1000,
        })
        .with_always_on_top(false);

    let config = Config::default().with_window(window_builder);

    log::info!("Initializing audio system...");
    // Initialize and leak the audio stream to keep it alive for the application lifetime
    let stream = match init_stream() {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to initialize audio stream: {}", e);
            show_error_dialog(&format!("Failed to initialize audio system:\n\n{}", e));
            std::process::exit(1);
        }
    };
    log::info!("Audio stream created, leaking to keep alive...");
    Box::leak(Box::new(stream));

    log::info!("Launching Dioxus application...");
    dioxus::LaunchBuilder::new().with_cfg(config).launch(App)
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::{create_dir_all, OpenOptions};

    // Try to create log file in a writable location
    let log_path = if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            std::path::PathBuf::from(home).join("Library/Logs/ChordFlow.log")
        } else {
            std::path::PathBuf::from("/tmp/chordflow.log")
        }
    } else {
        // Linux/other
        if let Some(home) = std::env::var_os("HOME") {
            std::path::PathBuf::from(home).join(".local/share/chordflow/chordflow.log")
        } else {
            std::path::PathBuf::from("/tmp/chordflow.log")
        }
    };

    // Ensure the parent directory exists
    if let Some(parent) = log_path.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter_level(log::LevelFilter::Info)
        .init();

    Ok(())
}

fn show_error_dialog(message: &str) {
    // Use native macOS dialog
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"display dialog "{}" buttons {{"OK"}} default button "OK" with icon stop with title "ChordFlow Error""#,
                message.replace('"', "\\\"").replace('\n', "\\n")
            ))
            .output();
    }

    #[cfg(not(target_os = "macos"))]
    {
        eprintln!("ERROR: {}", message);
    }
}
