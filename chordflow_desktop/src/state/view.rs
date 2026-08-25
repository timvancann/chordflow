use strum::{AsRefStr, Display, EnumCount, EnumIter, FromRepr};

/// Which top-level screen the window is showing. The practice screen and the
/// reference screen are alternatives, not layers: only one renders at a time.
/// Switching between them never touches audio, so the metronome keeps running.
#[derive(
    Clone, Copy, Debug, EnumIter, Display, AsRefStr, PartialEq, EnumCount, FromRepr, Default, Eq,
)]
pub enum View {
    #[default]
    Practice,
    Reference,
}
