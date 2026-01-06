use chordflow_music_theory::{
    chord::Chord,
    interval::Interval,
    note::{Note, NoteLetter},
    quality::Quality,
};

pub struct FourthsConfig {
    pub quality: Quality,
    pub current_chord: Chord,
    pub next_chord: Chord,
}

impl FourthsConfig {
    pub fn new(quality: Quality) -> Self {
        FourthsConfig {
            quality,
            current_chord: Chord::new(Note::new(NoteLetter::B, 0), quality),
            next_chord: Chord::new(Note::new(NoteLetter::E, 0), quality),
        }
    }

    pub fn reset(&mut self) {
        self.current_chord = Chord::new(Note::new(NoteLetter::B, 0), self.quality);
        self.next_chord = Chord::new(Note::new(NoteLetter::E, 0), self.quality);
    }
    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        let mut next_note = self
            .current_chord
            .root
            .add_interval(Interval::PerfectFourth);
        if next_note == Note::new(NoteLetter::G, -1) {
            next_note = Note::new(NoteLetter::F, 1);
        }
        self.next_chord = Chord::new(next_note, self.quality);
    }

    pub fn get_chords(&self) -> (String, String) {
        (self.current_chord.to_string(), self.next_chord.to_string())
    }
}

impl Default for FourthsConfig {
    fn default() -> Self {
        FourthsConfig {
            quality: Quality::Major,
            current_chord: Chord::new(Note::new(NoteLetter::B, 0), Quality::Major),
            next_chord: Chord::new(Note::new(NoteLetter::E, 0), Quality::Major),
        }
    }
}
