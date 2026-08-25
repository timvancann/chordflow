use anyhow::Result;

use chordparser::{chord::Chord, parsing::Parser};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProgressionConfig {
    pub chords: Vec<ProgressionChord>,
    pub current_chord: Option<Chord>,
    pub next_chord: Option<Chord>,
    pub current_chord_index: usize,
}

impl ProgressionConfig {
    pub fn get_chords(&self) -> (String, String) {
        (
            self.current_chord
                .clone()
                .map(|c| c.origin)
                .unwrap_or_default(),
            self.next_chord
                .clone()
                .map(|c| c.origin)
                .unwrap_or_default(),
        )
    }

    pub fn get_bars_per_cycle_current(&self) -> u8 {
        self.chords
            .get(self.current_chord_index)
            .map(|c| c.bars)
            .unwrap_or(1)
    }

    pub fn generate_next_chord(&mut self) {
        if self.chords.is_empty() {
            return;
        }
        self.current_chord = self.next_chord.clone();
        self.current_chord_index = (self.current_chord_index + 1) % self.chords.len();
        let next_index = (self.current_chord_index + 1) % self.chords.len();
        self.next_chord = Some(self.chords[next_index].chord.clone());
    }

    pub fn decrements_bars(&mut self, index: usize) {
        if let Some(chord) = self.chords.get_mut(index) {
            chord.bars = chord.bars.saturating_sub(1);
        }
    }

    pub fn increments_bars(&mut self, index: usize) {
        if let Some(chord) = self.chords.get_mut(index) {
            chord.bars += 1;
        }
    }

    pub fn reset(&mut self) {
        if self.chords.is_empty() {
            return;
        }
        if self.chords.len() < 2 {
            self.current_chord = Some(self.chords[0].chord.clone());
            self.next_chord = Some(self.chords[0].chord.clone());
            self.current_chord_index = 0;
            return;
        }
        self.current_chord = Some(self.chords[0].chord.clone());
        self.next_chord = Some(self.chords[1].chord.clone());
        self.current_chord_index = 0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressionChord {
    pub chord: Chord,
    pub bars: u8,
}

impl ProgressionChord {
    pub fn new(chord: Chord) -> Self {
        Self { chord, bars: 1 }
    }

    pub fn from_string(str: String) -> Result<Vec<ProgressionChord>> {
        let mut parser = Parser::new();
        str.split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let cleaned = s.replace(|c: char| c.is_whitespace() || c == ',', "");
                let chord = parser.parse(&cleaned)?;
                Ok(ProgressionChord::new(chord))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_next_chord_on_empty_progression_does_not_panic() {
        let mut config = ProgressionConfig::default();
        config.generate_next_chord();
        assert_eq!(config.current_chord_index, 0);
        assert!(config.next_chord.is_none());
    }

    #[test]
    fn get_bars_per_cycle_current_on_empty_progression_does_not_panic() {
        let config = ProgressionConfig::default();
        assert_eq!(config.get_bars_per_cycle_current(), 1);
    }

    #[test]
    fn generate_next_chord_cycles_through_progression() {
        let chords = ProgressionChord::from_string("C G Am F".to_string()).unwrap();
        let mut config = ProgressionConfig {
            chords,
            ..Default::default()
        };
        config.reset();
        config.generate_next_chord();
        assert_eq!(config.current_chord_index, 1);
        assert_eq!(config.current_chord.as_ref().unwrap().origin, "G");
        assert_eq!(config.next_chord.as_ref().unwrap().origin, "Am");
    }
}
