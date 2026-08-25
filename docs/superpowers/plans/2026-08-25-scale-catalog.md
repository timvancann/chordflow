# Scale Catalog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give `chordflow_music_theory` a tested catalog of the 27 scales from `Scales.pdf`, able to spell any of them in any key, name the chords that fit them, and answer the reverse question "which scales contain this chord?".

**Architecture:** Scales stay represented the way they already are, as a root plus an ordered `Vec<Interval>` formula. `Interval` gains the four spellings the poster needs; `ScaleType` grows from one variant into the 27-entry catalog with a `formula()` accessor; `Scale` gains note spelling and chord derivation; a free function inverts the lookup. No UI, no new dependencies. The only change outside the theory crate is a mechanical enum rename.

**Tech Stack:** Rust 2021, `strum` derives for enum iteration, `cargo test`. No new crates.

**Spec:** `docs/superpowers/specs/2026-08-25-scale-catalog-design.md`

## Global Constraints

- **`Scales.pdf` is the authority** for every formula and every scale name, with one documented exception: major blues is `R 2 b3 3 5 6`, not the `R 2 b3 b3 5 6` printed on the poster (confirmed typo).
- **Poster names are kept verbatim**, including "locrian #4" for `R 2 b3 #4 5 6 b7`, even where another name is more common.
- **No UI changes.** Nothing under `chordflow_desktop/src/ui/` is touched by any task in this plan.
- **Existing semitone values do not change.** Adding `Interval` variants must not alter what any existing variant returns from `to_semitones()` or `to_index()`.
- **Tests live inline** in `#[cfg(test)] mod tests` in the same file as the implementation, per this repo's convention (CLAUDE.md, "Testing").
- **Commit style:** one conventional-commit subject line, no body, no trailers.
- Run tests with `cargo test -p chordflow_music_theory` (or `just test` for the whole workspace).

---

## File Structure

| File | Responsibility | Tasks |
|---|---|---|
| `chordflow_music_theory/src/interval.rs` | Interval spellings, semitones, letter-steps, degree labels | 1 |
| `chordflow_music_theory/src/note.rs` | Unchanged implementation; its `test_add_interval` list is extended | 1 |
| `chordflow_music_theory/src/quality.rs` | Chord qualities, symbol/name/interval mapping | 2 |
| `chordflow_music_theory/src/scale.rs` | The catalog, note spelling, chord derivation, reverse lookup | 3, 4, 5, 6 |
| `chordflow_desktop/src/state/diatonic.rs` | Becomes a caller of the theory crate rather than an owner of the chord algorithm | 3, 5 |
| `chordflow_desktop/src/state/mode.rs`, `state/practice.rs` | Mechanical rename call sites only | 3 |

`scale.rs` carries four of the six tasks and will end up the largest file in the crate (roughly 400 lines with tests). That is deliberate: the catalog, the formulas, and the functions that interpret them change together, and splitting them would separate the data from its only consumers.

---

### Task 1: Interval gains four spellings and a degree label

**Files:**
- Modify: `chordflow_music_theory/src/interval.rs`
- Modify: `chordflow_music_theory/src/note.rs:145-172` (extend the existing `test_add_interval` expectation list)
- Test: `chordflow_music_theory/src/interval.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: nothing.
- Produces: `Interval::AugmentedSecond`, `Interval::DiminishedFourth`, `Interval::AugmentedFifth`, `Interval::DiminishedSeventh`, and `Interval::degree_label(self) -> &'static str`.

Background for the implementer: `Interval::to_semitones` gives pitch distance, `Interval::to_index` gives the *letter-step* (how many letter names to move: a third moves 2 letters whether major or minor). `Note::add_interval` (`note.rs:113`) uses both, which is why C plus a `DiminishedSeventh` spells `Bbb` and not `A`. Getting `to_index` right is what makes the whole catalog spell correctly.

New variants are inserted in musical order, not appended. That is safe: `Interval::from_repr` is used nowhere in the workspace, and `Interval::iter()` has exactly one consumer, the `test_add_interval` test.

- [ ] **Step 1: Write the failing tests**

Add to `chordflow_music_theory/src/interval.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn test_new_variants_semitones_and_letter_steps() {
        let cases = [
            (Interval::AugmentedSecond, 3, 1),
            (Interval::DiminishedFourth, 4, 3),
            (Interval::AugmentedFifth, 8, 4),
            (Interval::DiminishedSeventh, 9, 6),
        ];
        for (interval, semitones, letter_step) in cases {
            assert_eq!(interval.to_semitones(), semitones, "{interval:?} semitones");
            assert_eq!(interval.to_index(), letter_step, "{interval:?} letter step");
        }
    }

    #[test]
    fn test_existing_variants_are_unchanged() {
        assert_eq!(Interval::MinorThird.to_semitones(), 3);
        assert_eq!(Interval::MinorThird.to_index(), 2);
        assert_eq!(Interval::AugmentedFourth.to_semitones(), 6);
        assert_eq!(Interval::AugmentedFourth.to_index(), 3);
        assert_eq!(Interval::DiminishedFifth.to_semitones(), 6);
        assert_eq!(Interval::DiminishedFifth.to_index(), 4);
        assert_eq!(Interval::MinorSixth.to_semitones(), 8);
        assert_eq!(Interval::MinorSixth.to_index(), 5);
    }

    #[test]
    fn test_degree_labels_match_the_poster() {
        let cases = [
            (Interval::Unison, "R"),
            (Interval::MinorSecond, "b2"),
            (Interval::MajorSecond, "2"),
            (Interval::AugmentedSecond, "#2"),
            (Interval::MinorThird, "b3"),
            (Interval::MajorThird, "3"),
            (Interval::DiminishedFourth, "b4"),
            (Interval::PerfectFourth, "4"),
            (Interval::AugmentedFourth, "#4"),
            (Interval::DiminishedFifth, "b5"),
            (Interval::PerfectFifth, "5"),
            (Interval::AugmentedFifth, "#5"),
            (Interval::MinorSixth, "b6"),
            (Interval::MajorSixth, "6"),
            (Interval::DiminishedSeventh, "bb7"),
            (Interval::MinorSeventh, "b7"),
            (Interval::MajorSeventh, "7"),
        ];
        for (interval, label) in cases {
            assert_eq!(interval.degree_label(), label, "{interval:?}");
        }
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory interval`
Expected: FAIL to compile, `no variant named AugmentedSecond found for enum Interval`.

- [ ] **Step 3: Add the four variants in musical order**

Replace the `enum Interval` body in `chordflow_music_theory/src/interval.rs` with:

```rust
pub enum Interval {
    #[default]
    Unison,
    MinorSecond,
    MajorSecond,
    AugmentedSecond,
    MinorThird,
    MajorThird,
    DiminishedFourth,
    PerfectFourth,
    AugmentedFourth,
    Tritone,
    DiminishedFifth,
    PerfectFifth,
    AugmentedFifth,
    MinorSixth,
    MajorSixth,
    DiminishedSeventh,
    MinorSeventh,
    MajorSeventh,
    Octave,
}
```

- [ ] **Step 4: Add the new arms to `to_semitones` and `to_index`**

In `to_semitones`, add these arms (leave every existing arm exactly as it is):

```rust
            Interval::AugmentedSecond => 3,
            Interval::DiminishedFourth => 4,
            Interval::AugmentedFifth => 8,
            Interval::DiminishedSeventh => 9,
```

In `to_index`, add:

```rust
            Interval::AugmentedSecond => 1,
            Interval::DiminishedFourth => 3,
            Interval::AugmentedFifth => 4,
            Interval::DiminishedSeventh => 6,
```

Leave `from_semitone` alone. It is intentionally lossy (it maps 6 to `Tritone` and has no way to choose between `#4` and `b5`), it is only used by chord-quality code where spelling is irrelevant, and the catalog never calls it.

- [ ] **Step 5: Add `degree_label`**

Add to `impl Interval` in `chordflow_music_theory/src/interval.rs`:

```rust
    /// The scale-degree notation used on the Scales.pdf poster.
    pub fn degree_label(self) -> &'static str {
        match self {
            Interval::Unison | Interval::Octave => "R",
            Interval::MinorSecond => "b2",
            Interval::MajorSecond => "2",
            Interval::AugmentedSecond => "#2",
            Interval::MinorThird => "b3",
            Interval::MajorThird => "3",
            Interval::DiminishedFourth => "b4",
            Interval::PerfectFourth => "4",
            Interval::AugmentedFourth | Interval::Tritone => "#4",
            Interval::DiminishedFifth => "b5",
            Interval::PerfectFifth => "5",
            Interval::AugmentedFifth => "#5",
            Interval::MinorSixth => "b6",
            Interval::MajorSixth => "6",
            Interval::DiminishedSeventh => "bb7",
            Interval::MinorSeventh => "b7",
            Interval::MajorSeventh => "7",
        }
    }
```

- [ ] **Step 6: Extend the `test_add_interval` expectation list**

`note.rs:145` zips `Interval::iter()` against a fixed list of expected notes, so it must gain four entries in the right positions. Replace the `actual_notes` vector in `chordflow_music_theory/src/note.rs` with:

```rust
        let actual_notes = vec![
            Note::new(NoteLetter::C, 0),  // R
            Note::new(NoteLetter::D, -1), // b2
            Note::new(NoteLetter::D, 0),  // 2
            Note::new(NoteLetter::D, 1),  // #2
            Note::new(NoteLetter::E, -1), // b3
            Note::new(NoteLetter::E, 0),  // 3
            Note::new(NoteLetter::F, -1), // b4
            Note::new(NoteLetter::F, 0),  // 4
            Note::new(NoteLetter::F, 1),  // #4
            Note::new(NoteLetter::F, 1),  // tritone, spelled as #4
            Note::new(NoteLetter::G, -1), // b5
            Note::new(NoteLetter::G, 0),  // 5
            Note::new(NoteLetter::G, 1),  // #5
            Note::new(NoteLetter::A, -1), // b6
            Note::new(NoteLetter::A, 0),  // 6
            Note::new(NoteLetter::B, -2), // bb7
            Note::new(NoteLetter::B, -1), // b7
            Note::new(NoteLetter::B, 0),  // 7
        ];
```

The list stops before `Octave` exactly as it does today; `zip` ends at the shorter side, and `Interval::Octave.to_index()` panics by design.

- [ ] **Step 7: Run the tests**

Run: `cargo test -p chordflow_music_theory`
Expected: PASS, including `test_add_interval` and the three new interval tests.

- [ ] **Step 8: Commit**

```bash
git add chordflow_music_theory/src/interval.rs chordflow_music_theory/src/note.rs
git commit -m "feat(theory): add altered interval spellings and degree labels"
```

---

### Task 2: Quality gains three seventh chords and stops panicking

**Files:**
- Modify: `chordflow_music_theory/src/quality.rs`
- Modify: `chordflow_desktop/src/state/diatonic.rs:88-101` (adapt to the new return type)
- Test: `chordflow_music_theory/src/quality.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: nothing from Task 1.
- Produces: `Quality::DiminishedSeventh`, `Quality::MinorMajorSeventh`, `Quality::AugmentedMajorSeventh`, and the changed signature `Quality::from_intervals(intervals: Vec<i32>) -> Option<Quality>`.

Background: stacking thirds through the 21 seven-note scales produces chords the current eight-variant `Quality` cannot name. It also produces sets no conventional symbol names at all, which is why `from_intervals` must be able to say "I don't know" instead of panicking.

There is a live bug to fix here. `from_intervals` maps `[0, 5, 7]` to `Quality::Augmented` (`quality.rs:81`), but `to_intervals` defines augmented as `[0, 4, 8]` (`quality.rs:66`). `[0, 5, 7]` is a suspended fourth. The two functions disagree, and the mapping is simply wrong. Nothing hits it today because the major scale never yields an augmented triad, but harmonic minor's third degree does.

`[0, 5, 7]` is deliberately left unmapped rather than given a `Sus4` variant. Stacking thirds never produces a suspended chord, so a `Sus4` variant would have no producer.

- [ ] **Step 1: Write the failing tests**

Add to `chordflow_music_theory/src/quality.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::Quality;

    #[test]
    fn test_new_qualities_round_trip_through_intervals() {
        let cases = [
            (Quality::DiminishedSeventh, vec![0, 3, 6, 9]),
            (Quality::MinorMajorSeventh, vec![0, 3, 7, 11]),
            (Quality::AugmentedMajorSeventh, vec![0, 4, 8, 11]),
        ];
        for (quality, semitones) in cases {
            assert_eq!(
                Quality::from_intervals(semitones.clone()),
                Some(quality),
                "{semitones:?} should name {quality:?}"
            );
            let round_tripped: Vec<i32> =
                quality.to_intervals().iter().map(|i| i.to_semitones()).collect();
            assert_eq!(round_tripped, semitones, "{quality:?} to_intervals");
        }
    }

    #[test]
    fn test_augmented_triad_is_recognised() {
        // Regression: this used to be mapped from [0, 5, 7], which is a sus4,
        // and to_intervals disagreed with from_intervals as a result.
        assert_eq!(Quality::from_intervals(vec![0, 4, 8]), Some(Quality::Augmented));
        let augmented: Vec<i32> = Quality::Augmented
            .to_intervals()
            .iter()
            .map(|i| i.to_semitones())
            .collect();
        assert_eq!(augmented, vec![0, 4, 8]);
    }

    #[test]
    fn test_unnameable_sets_return_none_instead_of_panicking() {
        assert_eq!(Quality::from_intervals(vec![0, 5, 7]), None);
        assert_eq!(Quality::from_intervals(vec![0, 1, 4, 6]), None);
        assert_eq!(Quality::from_intervals(vec![]), None);
    }

    #[test]
    fn test_existing_qualities_still_map() {
        assert_eq!(Quality::from_intervals(vec![0, 4, 7]), Some(Quality::Major));
        assert_eq!(Quality::from_intervals(vec![0, 3, 7]), Some(Quality::Minor));
        assert_eq!(Quality::from_intervals(vec![0, 3, 6]), Some(Quality::Diminished));
        assert_eq!(Quality::from_intervals(vec![0, 4, 7, 10]), Some(Quality::Dominant));
        assert_eq!(Quality::from_intervals(vec![0, 3, 7, 10]), Some(Quality::MinorSeventh));
        assert_eq!(Quality::from_intervals(vec![0, 4, 7, 11]), Some(Quality::MajorSeventh));
        assert_eq!(Quality::from_intervals(vec![0, 3, 6, 10]), Some(Quality::HalfDiminished));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory quality`
Expected: FAIL to compile, `no variant named DiminishedSeventh found for enum Quality`.

- [ ] **Step 3: Add the three variants**

Append to the `enum Quality` body in `chordflow_music_theory/src/quality.rs`, after `HalfDiminished`:

```rust
    #[strum(to_string = "o7")]
    DiminishedSeventh,
    #[strum(to_string = "-Δ")]
    MinorMajorSeventh,
    #[strum(to_string = "+Δ")]
    AugmentedMajorSeventh,
```

- [ ] **Step 4: Extend the four mapping functions**

In `from_string`, add before the `_ =>` arm:

```rust
            "dim7" => Quality::DiminishedSeventh,
            "o7" => Quality::DiminishedSeventh,
            "mMaj7" => Quality::MinorMajorSeventh,
            "maj7#5" => Quality::AugmentedMajorSeventh,
```

In `from_name`, add before the `_ =>` arm:

```rust
            "Diminished Seventh" => Quality::DiminishedSeventh,
            "Minor Major Seventh" => Quality::MinorMajorSeventh,
            "Augmented Major Seventh" => Quality::AugmentedMajorSeventh,
```

In `name`, add:

```rust
            Quality::DiminishedSeventh => "Diminished Seventh",
            Quality::MinorMajorSeventh => "Minor Major Seventh",
            Quality::AugmentedMajorSeventh => "Augmented Major Seventh",
```

In `to_intervals`, add:

```rust
            Quality::DiminishedSeventh => Interval::from_semitones([0, 3, 6, 9].to_vec()),
            Quality::MinorMajorSeventh => Interval::from_semitones([0, 3, 7, 11].to_vec()),
            Quality::AugmentedMajorSeventh => Interval::from_semitones([0, 4, 8, 11].to_vec()),
```

- [ ] **Step 5: Fix and re-type `from_intervals`**

Replace `from_intervals` in `chordflow_music_theory/src/quality.rs` with:

```rust
    /// Names a root-relative semitone set, or `None` if no conventional
    /// chord symbol describes it. Returning `None` rather than guessing is
    /// deliberate: stacking thirds through the exotic scales produces sets
    /// that no symbol names, and a wrong chord symbol is worse than none.
    pub fn from_intervals(intervals: Vec<i32>) -> Option<Quality> {
        match intervals[..] {
            [0, 4, 7] => Some(Quality::Major),
            [0, 3, 7] => Some(Quality::Minor),
            [0, 3, 6] => Some(Quality::Diminished),
            [0, 4, 8] => Some(Quality::Augmented),
            [0, 4, 7, 10] => Some(Quality::Dominant),
            [0, 3, 7, 10] => Some(Quality::MinorSeventh),
            [0, 4, 7, 11] => Some(Quality::MajorSeventh),
            [0, 3, 6, 10] => Some(Quality::HalfDiminished),
            [0, 3, 6, 9] => Some(Quality::DiminishedSeventh),
            [0, 3, 7, 11] => Some(Quality::MinorMajorSeventh),
            [0, 4, 8, 11] => Some(Quality::AugmentedMajorSeventh),
            _ => None,
        }
    }
```

Note the `[0, 5, 7] => Augmented` arm is gone, replaced by the correct `[0, 4, 8] => Augmented`.

- [ ] **Step 6: Adapt the one existing caller**

`calculate_chord_quality_in_scale` in `chordflow_desktop/src/state/diatonic.rs:88` returns `Quality`. It only ever runs on the major scale, where every degree is nameable, so it keeps that signature and asserts. Change its last line from:

```rust
    Quality::from_intervals(zero_based_chord_indexes)
```

to:

```rust
    Quality::from_intervals(zero_based_chord_indexes.clone()).unwrap_or_else(|| {
        panic!("no chord quality names the interval set {zero_based_chord_indexes:?}")
    })
```

This preserves today's behaviour (it panicked before too) while naming the offending set instead of saying "Invalid intervals". Task 5 replaces this function entirely.

- [ ] **Step 7: Run the tests**

Run: `cargo test`
Expected: PASS across the whole workspace, including the existing `practice.rs` tests that exercise diatonic chord quality.

- [ ] **Step 8: Commit**

```bash
git add chordflow_music_theory/src/quality.rs chordflow_desktop/src/state/diatonic.rs
git commit -m "fix(theory): correct the augmented triad mapping and name seventh chords from the modes"
```

---

### Task 3: ScaleType becomes the 27-scale catalog

**Files:**
- Modify: `chordflow_music_theory/src/scale.rs`
- Modify: `chordflow_desktop/src/state/diatonic.rs:20,56,58`
- Modify: `chordflow_desktop/src/state/mode.rs:45`
- Modify: `chordflow_desktop/src/state/practice.rs:141,148,154,174`
- Test: `chordflow_music_theory/src/scale.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: `Interval::AugmentedSecond`, `Interval::DiminishedFourth`, `Interval::AugmentedFifth`, `Interval::DiminishedSeventh`, `Interval::degree_label` from Task 1.
- Produces: `ScaleFamily` (`Major | MelodicMinor | HarmonicMinor | Other`), the 27 `ScaleType` variants, `ScaleType::family(self) -> ScaleFamily`, `ScaleType::formula(self) -> Vec<Interval>`, `ScaleType::display_name(self) -> &'static str`. `ScaleType::Diatonic` no longer exists; it is `ScaleType::Ionian`.

Background: `Scale::new` currently matches on the single `Diatonic` variant to build a hardcoded interval vector (`scale.rs:29`). After this task it just reads `scale_type.formula()`. `Scale`'s own struct shape does not change.

The strum `Display` derive on `ScaleType` is replaced by a manual `impl Display` delegating to `display_name`, so the poster's lowercase names ("lydian dominant") are what prints, without duplicating the name table. This is safe: `Scale`'s `Display` impl is the only consumer, and no UI code formats a `Scale` (the UI only reads `.scale.root`).

- [ ] **Step 1: Write the failing tests**

Replace the existing `mod tests` in `chordflow_music_theory/src/scale.rs` with:

```rust
#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::note::{Note, NoteLetter};

    use super::{Scale, ScaleFamily, ScaleType};

    /// Every row of Scales.pdf, transcribed. This table is the oracle for the
    /// whole catalog: if a formula is ever mistyped, this test fails.
    fn poster() -> Vec<(ScaleType, ScaleFamily, &'static str, &'static str)> {
        vec![
            (ScaleType::Ionian, ScaleFamily::Major, "ionian", "R 2 3 4 5 6 7"),
            (ScaleType::Dorian, ScaleFamily::Major, "dorian", "R 2 b3 4 5 6 b7"),
            (ScaleType::Phrygian, ScaleFamily::Major, "phrygian", "R b2 b3 4 5 b6 b7"),
            (ScaleType::Lydian, ScaleFamily::Major, "lydian", "R 2 3 #4 5 6 7"),
            (ScaleType::Mixolydian, ScaleFamily::Major, "mixolydian", "R 2 3 4 5 6 b7"),
            (ScaleType::Aeolian, ScaleFamily::Major, "aeolian", "R 2 b3 4 5 b6 b7"),
            (ScaleType::Locrian, ScaleFamily::Major, "locrian", "R b2 b3 4 b5 b6 b7"),

            (ScaleType::MelodicMinor, ScaleFamily::MelodicMinor, "melodic minor", "R 2 b3 4 5 6 7"),
            (ScaleType::DorianFlat2, ScaleFamily::MelodicMinor, "dorian b2", "R b2 b3 4 5 6 b7"),
            (ScaleType::LydianAugmented, ScaleFamily::MelodicMinor, "lydian augmented", "R 2 3 #4 #5 6 7"),
            (ScaleType::LydianDominant, ScaleFamily::MelodicMinor, "lydian dominant", "R 2 3 #4 5 6 b7"),
            (ScaleType::MixolydianFlat6, ScaleFamily::MelodicMinor, "mixolydian b6", "R 2 3 4 5 b6 b7"),
            (ScaleType::AeolianFlat5, ScaleFamily::MelodicMinor, "aeolian b5", "R 2 b3 4 b5 b6 b7"),
            (ScaleType::Altered, ScaleFamily::MelodicMinor, "altered", "R b2 #2 3 b5 #5 b7"),

            (ScaleType::HarmonicMinor, ScaleFamily::HarmonicMinor, "harmonic minor", "R 2 b3 4 5 b6 7"),
            (ScaleType::LocrianNatural6, ScaleFamily::HarmonicMinor, "locrian natural 6", "R b2 b3 4 b5 6 b7"),
            (ScaleType::IonianAugmented, ScaleFamily::HarmonicMinor, "ionian augmented", "R 2 3 4 #5 6 7"),
            (ScaleType::LocrianSharp4, ScaleFamily::HarmonicMinor, "locrian #4", "R 2 b3 #4 5 6 b7"),
            (ScaleType::PhrygianDominant, ScaleFamily::HarmonicMinor, "phrygian dominant", "R b2 3 4 5 b6 b7"),
            (ScaleType::LydianSharp9, ScaleFamily::HarmonicMinor, "lydian #9", "R #2 3 #4 5 6 7"),
            (ScaleType::SuperlocrianDoubleFlat7, ScaleFamily::HarmonicMinor, "superlocrian bb7", "R b2 b3 b4 b5 b6 bb7"),

            // major blues: the poster prints "R 2 b3 b3 5 6"; confirmed typo,
            // the flat third is followed by the natural third.
            (ScaleType::MajorBlues, ScaleFamily::Other, "major blues", "R 2 b3 3 5 6"),
            (ScaleType::MinorBlues, ScaleFamily::Other, "minor blues", "R b3 4 b5 5 b7"),
            (ScaleType::WholeTone, ScaleFamily::Other, "whole tone", "R 2 3 #4 #5 b7"),
            (ScaleType::Augmented, ScaleFamily::Other, "augmented", "R #2 3 5 #5 7"),
            (ScaleType::DiminishedHalfWhole, ScaleFamily::Other, "diminished half whole", "R b2 #2 3 b5 5 6 b7"),
            (ScaleType::DiminishedWholeHalf, ScaleFamily::Other, "diminished whole half", "R 2 b3 4 b5 #5 6 7"),
        ]
    }

    #[test]
    fn test_formulas_match_the_poster() {
        for (scale_type, _, name, formula) in poster() {
            let actual: Vec<&str> = scale_type
                .formula()
                .iter()
                .map(|i| i.degree_label())
                .collect();
            assert_eq!(actual.join(" "), formula, "{name} formula");
        }
    }

    #[test]
    fn test_families_and_names_match_the_poster() {
        for (scale_type, family, name, _) in poster() {
            assert_eq!(scale_type.family(), family, "{name} family");
            assert_eq!(scale_type.display_name(), name, "{scale_type:?} name");
        }
    }

    #[test]
    fn test_catalog_is_complete_and_has_no_extras() {
        let listed: Vec<ScaleType> = poster().into_iter().map(|(t, _, _, _)| t).collect();
        let all: Vec<ScaleType> = ScaleType::iter().collect();
        assert_eq!(all.len(), 27, "the poster has 27 scales");
        assert_eq!(all, listed, "ScaleType order must match the poster's order");
    }

    #[test]
    fn test_scale_new_reads_the_formula() {
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Lydian);
        assert_eq!(scale.intervals, ScaleType::Lydian.formula());
        assert_eq!(scale.root, Note::new(NoteLetter::C, 0));
    }

    #[test]
    fn test_display_uses_the_poster_name() {
        let scale = Scale::new(Note::new(NoteLetter::G, 0), ScaleType::LydianDominant);
        assert_eq!(scale.to_string(), "G lydian dominant");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory scale`
Expected: FAIL to compile, `no variant named Ionian found for enum ScaleType`.

- [ ] **Step 3: Write the catalog**

Replace everything above the `#[cfg(test)]` block in `chordflow_music_theory/src/scale.rs` with:

```rust
use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter, FromRepr};

use super::{interval::Interval, note::Note};

/// The four groupings used on the Scales.pdf poster.
#[derive(Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, Eq)]
pub enum ScaleFamily {
    Major,
    MelodicMinor,
    HarmonicMinor,
    Other,
}

/// Every scale on the Scales.pdf poster, in the poster's order.
#[derive(
    Default, Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq, EnumCount, FromRepr, Eq,
)]
pub enum ScaleType {
    #[default]
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,

    MelodicMinor,
    DorianFlat2,
    LydianAugmented,
    LydianDominant,
    MixolydianFlat6,
    AeolianFlat5,
    Altered,

    HarmonicMinor,
    LocrianNatural6,
    IonianAugmented,
    LocrianSharp4,
    PhrygianDominant,
    LydianSharp9,
    SuperlocrianDoubleFlat7,

    MajorBlues,
    MinorBlues,
    WholeTone,
    Augmented,
    DiminishedHalfWhole,
    DiminishedWholeHalf,
}

impl Display for ScaleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl ScaleType {
    pub fn family(self) -> ScaleFamily {
        use ScaleType::*;
        match self {
            Ionian | Dorian | Phrygian | Lydian | Mixolydian | Aeolian | Locrian => {
                ScaleFamily::Major
            }
            MelodicMinor | DorianFlat2 | LydianAugmented | LydianDominant | MixolydianFlat6
            | AeolianFlat5 | Altered => ScaleFamily::MelodicMinor,
            HarmonicMinor | LocrianNatural6 | IonianAugmented | LocrianSharp4
            | PhrygianDominant | LydianSharp9 | SuperlocrianDoubleFlat7 => {
                ScaleFamily::HarmonicMinor
            }
            MajorBlues | MinorBlues | WholeTone | Augmented | DiminishedHalfWhole
            | DiminishedWholeHalf => ScaleFamily::Other,
        }
    }

    /// The name as printed on the poster, kept verbatim.
    pub fn display_name(self) -> &'static str {
        use ScaleType::*;
        match self {
            Ionian => "ionian",
            Dorian => "dorian",
            Phrygian => "phrygian",
            Lydian => "lydian",
            Mixolydian => "mixolydian",
            Aeolian => "aeolian",
            Locrian => "locrian",
            MelodicMinor => "melodic minor",
            DorianFlat2 => "dorian b2",
            LydianAugmented => "lydian augmented",
            LydianDominant => "lydian dominant",
            MixolydianFlat6 => "mixolydian b6",
            AeolianFlat5 => "aeolian b5",
            Altered => "altered",
            HarmonicMinor => "harmonic minor",
            LocrianNatural6 => "locrian natural 6",
            IonianAugmented => "ionian augmented",
            LocrianSharp4 => "locrian #4",
            PhrygianDominant => "phrygian dominant",
            LydianSharp9 => "lydian #9",
            SuperlocrianDoubleFlat7 => "superlocrian bb7",
            MajorBlues => "major blues",
            MinorBlues => "minor blues",
            WholeTone => "whole tone",
            Augmented => "augmented",
            DiminishedHalfWhole => "diminished half whole",
            DiminishedWholeHalf => "diminished whole half",
        }
    }

    /// The scale's degrees, spelled. Spelling matters: `#4` and `b5` are the
    /// same fret but different letters, and only the right one makes
    /// `Note::add_interval` produce the correct note name.
    pub fn formula(self) -> Vec<Interval> {
        use Interval::*;
        use ScaleType::*;
        match self {
            Ionian => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh],
            Dorian => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            Phrygian => vec![Unison, MinorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            Lydian => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MajorSeventh],
            Mixolydian => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            Aeolian => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            Locrian => vec![Unison, MinorSecond, MinorThird, PerfectFourth, DiminishedFifth, MinorSixth, MinorSeventh],

            MelodicMinor => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MajorSeventh],
            DorianFlat2 => vec![Unison, MinorSecond, MinorThird, PerfectFourth, PerfectFifth, MajorSixth, MinorSeventh],
            LydianAugmented => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, AugmentedFifth, MajorSixth, MajorSeventh],
            LydianDominant => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MinorSeventh],
            MixolydianFlat6 => vec![Unison, MajorSecond, MajorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            AeolianFlat5 => vec![Unison, MajorSecond, MinorThird, PerfectFourth, DiminishedFifth, MinorSixth, MinorSeventh],
            Altered => vec![Unison, MinorSecond, AugmentedSecond, MajorThird, DiminishedFifth, AugmentedFifth, MinorSeventh],

            HarmonicMinor => vec![Unison, MajorSecond, MinorThird, PerfectFourth, PerfectFifth, MinorSixth, MajorSeventh],
            LocrianNatural6 => vec![Unison, MinorSecond, MinorThird, PerfectFourth, DiminishedFifth, MajorSixth, MinorSeventh],
            IonianAugmented => vec![Unison, MajorSecond, MajorThird, PerfectFourth, AugmentedFifth, MajorSixth, MajorSeventh],
            LocrianSharp4 => vec![Unison, MajorSecond, MinorThird, AugmentedFourth, PerfectFifth, MajorSixth, MinorSeventh],
            PhrygianDominant => vec![Unison, MinorSecond, MajorThird, PerfectFourth, PerfectFifth, MinorSixth, MinorSeventh],
            LydianSharp9 => vec![Unison, AugmentedSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth, MajorSeventh],
            SuperlocrianDoubleFlat7 => vec![Unison, MinorSecond, MinorThird, DiminishedFourth, DiminishedFifth, MinorSixth, DiminishedSeventh],

            MajorBlues => vec![Unison, MajorSecond, MinorThird, MajorThird, PerfectFifth, MajorSixth],
            MinorBlues => vec![Unison, MinorThird, PerfectFourth, DiminishedFifth, PerfectFifth, MinorSeventh],
            WholeTone => vec![Unison, MajorSecond, MajorThird, AugmentedFourth, AugmentedFifth, MinorSeventh],
            Augmented => vec![Unison, AugmentedSecond, MajorThird, PerfectFifth, AugmentedFifth, MajorSeventh],
            DiminishedHalfWhole => vec![Unison, MinorSecond, AugmentedSecond, MajorThird, DiminishedFifth, PerfectFifth, MajorSixth, MinorSeventh],
            DiminishedWholeHalf => vec![Unison, MajorSecond, MinorThird, PerfectFourth, DiminishedFifth, AugmentedFifth, MajorSixth, MajorSeventh],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scale {
    pub root: Note,
    pub scale_type: ScaleType,
    pub intervals: Vec<Interval>,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.root, self.scale_type)
    }
}

impl Scale {
    pub fn new(root: Note, scale_type: ScaleType) -> Scale {
        Scale {
            root,
            scale_type,
            intervals: scale_type.formula(),
        }
    }
}
```

- [ ] **Step 4: Rename the eight `Diatonic` call sites**

Run this and check the diff before committing:

```bash
sed -i '' 's/ScaleType::Diatonic/ScaleType::Ionian/g' \
  chordflow_desktop/src/state/diatonic.rs \
  chordflow_desktop/src/state/mode.rs \
  chordflow_desktop/src/state/practice.rs
git diff --stat
```

Expected: three files changed, eight lines touched.

- [ ] **Step 5: Run the tests**

Run: `cargo test`
Expected: PASS across the workspace. The `test_c_major_notes` test that used to live in `scale.rs` is intentionally gone; Task 4 replaces it with far broader spelling coverage.

- [ ] **Step 6: Commit**

```bash
git add chordflow_music_theory/src/scale.rs chordflow_desktop/src/state/
git commit -m "feat(theory): add the 27-scale catalog from the Scales poster"
```

---

### Task 4: Spell a scale's notes in any key

**Files:**
- Modify: `chordflow_music_theory/src/scale.rs`
- Test: `chordflow_music_theory/src/scale.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: `ScaleType::formula` from Task 3.
- Produces: `Scale::notes(&self) -> Vec<Note>`.

Background: this is the capability the paper poster cannot have. The poster shows `R 2 3 #4 5 6 7` for lydian; the app shows that G lydian is `G A B C# D E F#`. `Note::add_interval` (`note.rs:113`) already does the work correctly, including double accidentals, because it moves letter names and semitones independently.

- [ ] **Step 1: Write the failing tests**

Add these tests inside the existing `mod tests` in `chordflow_music_theory/src/scale.rs`:

```rust
    fn spell(root: (NoteLetter, i32), scale_type: ScaleType) -> String {
        Scale::new(Note::new(root.0, root.1), scale_type)
            .notes()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_spelling_in_easy_keys() {
        assert_eq!(spell((NoteLetter::C, 0), ScaleType::Ionian), "C D E F G A B");
        assert_eq!(spell((NoteLetter::G, 0), ScaleType::Lydian), "G A B C♯ D E F♯");
        assert_eq!(spell((NoteLetter::D, 0), ScaleType::Dorian), "D E F G A B C");
        assert_eq!(spell((NoteLetter::A, 0), ScaleType::HarmonicMinor), "A B C D E F G♯");
    }

    #[test]
    fn test_spelling_needs_sharps_that_look_wrong_but_are_right() {
        // F# major really does contain E#, not F.
        assert_eq!(spell((NoteLetter::F, 1), ScaleType::Ionian), "F♯ G♯ A♯ B C♯ D♯ E♯");
    }

    #[test]
    fn test_spelling_needs_double_flats() {
        // superlocrian bb7 is the scale that forces a doubly-flattened seventh.
        assert_eq!(
            spell((NoteLetter::C, 0), ScaleType::SuperlocrianDoubleFlat7),
            "C D♭ E♭ F♭ G♭ A♭ B♭♭"
        );
    }

    #[test]
    fn test_every_scale_spells_in_every_root_without_panicking() {
        for root in crate::note::generate_all_roots() {
            for scale_type in ScaleType::iter() {
                let scale = Scale::new(root, scale_type);
                assert_eq!(
                    scale.notes().len(),
                    scale_type.formula().len(),
                    "{root} {scale_type}"
                );
            }
        }
    }
```

The test module's `use` line needs `use crate::note::{Note, NoteLetter};` (already present from Task 3) and `use strum::IntoEnumIterator;` (already present from Task 3).

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory scale`
Expected: FAIL to compile, `no method named notes found for struct Scale`.

- [ ] **Step 3: Implement `notes`**

Add to `impl Scale` in `chordflow_music_theory/src/scale.rs`:

```rust
    /// The scale's notes, spelled correctly for this root. The same formula
    /// gives different letters per key: G lydian is G A B C# D E F#, while
    /// F# ionian genuinely contains E#.
    pub fn notes(&self) -> Vec<Note> {
        self.intervals
            .iter()
            .map(|interval| self.root.add_interval(*interval))
            .collect()
    }
```

- [ ] **Step 4: Run the tests**

Run: `cargo test -p chordflow_music_theory`
Expected: PASS. Note that `Note`'s `Display` uses the Unicode `♯` and `♭` characters (`note.rs:91`), which is why the expected strings above use them.

- [ ] **Step 5: Commit**

```bash
git add chordflow_music_theory/src/scale.rs
git commit -m "feat(theory): spell any catalog scale in any key"
```

---

### Task 5: Derive the chords that fit a scale

**Files:**
- Modify: `chordflow_music_theory/src/scale.rs`
- Modify: `chordflow_desktop/src/state/diatonic.rs:88-106` (delete the local algorithm, call the crate)
- Test: `chordflow_music_theory/src/scale.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: `Scale::notes` from Task 4, `Quality::from_intervals -> Option<Quality>` from Task 2.
- Produces: `Scale::diatonic_triads(&self) -> Option<Vec<Chord>>` and `Scale::diatonic_sevenths(&self) -> Option<Vec<Chord>>`.

Background: `calculate_chord_quality_in_scale` in `chordflow_desktop/src/state/diatonic.rs:88` already does this for triads over the major scale, but it hardcodes the seven-note assumption in `normalize(i, 7)` and lives in the wrong crate. This task generalizes it, adds the sevenths variant, and moves it into the theory crate.

Both functions return `None` for the six non-heptatonic scales (major blues, minor blues, whole tone, augmented, and both diminished scales), where stacking by scale index has no accepted meaning. Degrees whose interval set no `Quality` names are skipped rather than mislabelled, so a returned vector can be shorter than the scale.

- [ ] **Step 1: Write the failing tests**

Add inside `mod tests` in `chordflow_music_theory/src/scale.rs`:

```rust
    fn chord_symbols(chords: Option<Vec<crate::chord::Chord>>) -> String {
        chords
            .expect("expected a heptatonic scale")
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_triads_of_c_major() {
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Ionian);
        assert_eq!(chord_symbols(scale.diatonic_triads()), "C D- E- F G A- Bo");
    }

    #[test]
    fn test_sevenths_of_c_major() {
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::Ionian);
        assert_eq!(chord_symbols(scale.diatonic_sevenths()), "CΔ D-7 E-7 FΔ G7 A-7 Bø");
    }

    #[test]
    fn test_harmonic_minor_third_degree_is_augmented() {
        // This is the case that panics under the pre-Task-2 from_intervals.
        let scale = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::HarmonicMinor);
        let triads = scale.diatonic_triads().expect("heptatonic");
        assert_eq!(triads[2].quality, crate::quality::Quality::Augmented);
        assert_eq!(triads[2].root, Note::new(NoteLetter::E, -1));
    }

    #[test]
    fn test_the_three_new_qualities_have_producers() {
        use crate::quality::Quality;

        let melodic = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::MelodicMinor);
        assert_eq!(
            melodic.diatonic_sevenths().expect("heptatonic")[0].quality,
            Quality::MinorMajorSeventh
        );

        let harmonic = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::HarmonicMinor);
        assert_eq!(
            harmonic.diatonic_sevenths().expect("heptatonic")[6].quality,
            Quality::DiminishedSeventh
        );

        let lydian_aug = Scale::new(Note::new(NoteLetter::C, 0), ScaleType::LydianAugmented);
        assert_eq!(
            lydian_aug.diatonic_sevenths().expect("heptatonic")[0].quality,
            Quality::AugmentedMajorSeventh
        );
    }

    #[test]
    fn test_non_heptatonic_scales_have_no_diatonic_chords() {
        let non_heptatonic = [
            ScaleType::MajorBlues,
            ScaleType::MinorBlues,
            ScaleType::WholeTone,
            ScaleType::Augmented,
            ScaleType::DiminishedHalfWhole,
            ScaleType::DiminishedWholeHalf,
        ];
        for scale_type in non_heptatonic {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            assert!(scale.diatonic_triads().is_none(), "{scale_type} triads");
            assert!(scale.diatonic_sevenths().is_none(), "{scale_type} sevenths");
        }
    }

    #[test]
    fn test_every_heptatonic_scale_derives_chords_without_panicking() {
        for scale_type in ScaleType::iter().filter(|t| t.formula().len() == 7) {
            let scale = Scale::new(Note::new(NoteLetter::C, 0), scale_type);
            assert!(scale.diatonic_triads().is_some(), "{scale_type} triads");
            assert!(scale.diatonic_sevenths().is_some(), "{scale_type} sevenths");
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory scale`
Expected: FAIL to compile, `no method named diatonic_triads found for struct Scale`.

- [ ] **Step 3: Implement both functions and their shared helper**

Add to `impl Scale` in `chordflow_music_theory/src/scale.rs`:

```rust
    /// The triads built by stacking thirds on each degree, or `None` if this
    /// scale does not have seven notes (stacking by scale index has no
    /// accepted meaning for the blues, whole tone, augmented, and diminished
    /// scales).
    pub fn diatonic_triads(&self) -> Option<Vec<Chord>> {
        self.stacked_chords(&[0, 2, 4])
    }

    /// The seventh chords built by stacking thirds on each degree. Same
    /// `None` condition as `diatonic_triads`.
    pub fn diatonic_sevenths(&self) -> Option<Vec<Chord>> {
        self.stacked_chords(&[0, 2, 4, 6])
    }

    /// Builds one chord per degree by taking the scale members at the given
    /// index offsets. Degrees whose interval set no `Quality` names are
    /// skipped: a missing chord symbol is better than a wrong one, and the
    /// exotic scales produce several sets that no symbol describes.
    fn stacked_chords(&self, offsets: &[usize]) -> Option<Vec<Chord>> {
        if self.intervals.len() != 7 {
            return None;
        }

        let notes = self.notes();
        let chords = (0..7)
            .filter_map(|degree| {
                let members: Vec<i32> = offsets
                    .iter()
                    .map(|offset| self.intervals[(degree + offset) % 7].to_semitones())
                    .collect();
                let root_semitones = members[0];
                let relative: Vec<i32> = members
                    .iter()
                    .map(|s| (s - root_semitones).rem_euclid(12))
                    .collect();
                Quality::from_intervals(relative).map(|q| Chord::new(notes[degree], q))
            })
            .collect();

        Some(chords)
    }
```

Update the imports at the top of `chordflow_music_theory/src/scale.rs`:

```rust
use super::{chord::Chord, interval::Interval, note::Note, quality::Quality};
```

- [ ] **Step 4: Run the tests**

Run: `cargo test -p chordflow_music_theory`
Expected: PASS.

- [ ] **Step 5: Delete the desktop crate's copy of the algorithm**

In `chordflow_desktop/src/state/diatonic.rs`, delete the `calculate_chord_quality_in_scale` and `normalize` functions entirely (lines 88 to the end of `normalize`), and replace the two call sites (`diatonic.rs:27` and `diatonic.rs:42`) so they read the chord from the scale instead of recomputing the quality.

Replace `preview_next_chord`:

```rust
    fn preview_next_chord(&self) -> Chord {
        let interval = next_diatonic_scale_interval(self.is_random, &self.scale, &Interval::Unison);
        self.chord_at(interval)
    }
```

Replace the body of `generate_next_chord`:

```rust
    pub fn generate_next_chord(&mut self) {
        self.current_chord = self.next_chord;
        let interval =
            next_diatonic_scale_interval(self.is_random, &self.scale, &self.next_scale_interval);
        self.next_scale_interval = interval;
        self.next_chord = self.chord_at(interval);
    }
```

And add this helper to the same `impl DiatonicConfig`:

```rust
    /// The diatonic triad on the degree at `interval` within the current scale.
    fn chord_at(&self, interval: Interval) -> Chord {
        let degree = self
            .scale
            .intervals
            .iter()
            .position(|i| *i == interval)
            .expect("interval came from this scale");
        self.scale
            .diatonic_triads()
            .expect("the diatonic practice mode always uses a seven-note scale")[degree]
    }
```

Remove the now-unused `Quality` import from `chordflow_desktop/src/state/diatonic.rs` if the compiler flags it, and keep the `Quality::Major` uses in `set_root` and `reset` if they are still there.

- [ ] **Step 6: Run the whole workspace test suite**

Run: `cargo test`
Expected: PASS, including the `practice.rs` tests that assert diatonic chord sequences. If a `practice.rs` test imported `calculate_chord_quality_in_scale`, point it at `Scale::diatonic_triads` instead.

- [ ] **Step 7: Check for warnings**

Run: `cargo check`
Expected: no warnings about unused imports or dead code in `diatonic.rs`.

- [ ] **Step 8: Commit**

```bash
git add chordflow_music_theory/src/scale.rs chordflow_desktop/src/state/diatonic.rs
git commit -m "feat(theory): derive triads and sevenths from any heptatonic scale"
```

---

### Task 6: Reverse lookup, which scales contain this chord

**Files:**
- Modify: `chordflow_music_theory/src/scale.rs`
- Test: `chordflow_music_theory/src/scale.rs` (inline `mod tests`)

**Interfaces:**
- Consumes: `Scale::notes` from Task 4, `ScaleType::iter` from Task 3.
- Produces: `scale::scales_containing(chord: &Chord) -> Vec<Scale>` (a free function, not a method).

Background: every other function in this crate runs scale to chords. This one runs chord to scales, and it is what follow mode needs. Bmaj7 comes up in a drill, and the player wants to know which scales they can improvise with over it.

The subtlety is spelling. `note::generate_all_roots()` returns **17** spellings, not 12: seven letters times three accidentals, minus the four it filters out at `note.rs:159` (Cb, E#, B#, Fb). So C# and Db are both in the list, and the same scale would appear twice under two names. Matches are deduplicated by `(scale_type, root pitch class)`, keeping the spelling whose notes carry the fewest total accidentals, ties broken toward the flat spelling. That yields Bb mixolydian rather than A# mixolydian.

- [ ] **Step 1: Write the failing tests**

Add inside `mod tests` in `chordflow_music_theory/src/scale.rs`:

```rust
    use crate::chord::Chord;
    use crate::quality::Quality;

    use super::scales_containing;

    fn contains_scale(scales: &[Scale], root: (NoteLetter, i32), scale_type: ScaleType) -> bool {
        scales
            .iter()
            .any(|s| s.root == Note::new(root.0, root.1) && s.scale_type == scale_type)
    }

    #[test]
    fn test_finds_the_obvious_homes_of_c_major_seventh() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        assert!(contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Ionian));
        assert!(contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Lydian));
        assert!(contains_scale(&scales, (NoteLetter::G, 0), ScaleType::Ionian));
    }

    #[test]
    fn test_excludes_scales_that_do_not_contain_the_chord() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        // C phrygian has Eb and Bb; it cannot host a C major seventh.
        assert!(!contains_scale(&scales, (NoteLetter::C, 0), ScaleType::Phrygian));
    }

    #[test]
    fn test_results_are_deduplicated_by_pitch_class() {
        let cmaj7 = Chord::new(Note::new(NoteLetter::C, 0), Quality::MajorSeventh);
        let scales = scales_containing(&cmaj7);

        let mut seen: Vec<(ScaleType, i32)> = scales
            .iter()
            .map(|s| (s.scale_type, s.root.to_semitones().rem_euclid(12)))
            .collect();
        let before = seen.len();
        seen.sort_by_key(|(t, pc)| (*t as usize, *pc));
        seen.dedup();
        assert_eq!(before, seen.len(), "no scale should appear twice");
    }

    #[test]
    fn test_prefers_the_simpler_spelling() {
        // F7 is F A C Eb, so it lives in Bb ionian (F is its fifth degree).
        // A# ionian is the same pitch classes spelled with far more
        // accidentals, so the tiebreak must reject it.
        let f7 = Chord::new(Note::new(NoteLetter::F, 0), Quality::Dominant);
        let scales = scales_containing(&f7);

        assert!(contains_scale(&scales, (NoteLetter::B, -1), ScaleType::Ionian));
        assert!(!contains_scale(&scales, (NoteLetter::A, 1), ScaleType::Ionian));
    }

    #[test]
    fn test_never_empty_for_any_chord_the_practice_modes_can_produce() {
        let qualities = [
            Quality::Major,
            Quality::Minor,
            Quality::Diminished,
            Quality::Augmented,
            Quality::Dominant,
            Quality::MajorSeventh,
            Quality::MinorSeventh,
            Quality::HalfDiminished,
        ];
        for root in crate::note::generate_all_roots() {
            for quality in qualities {
                let chord = Chord::new(root, quality);
                assert!(
                    !scales_containing(&chord).is_empty(),
                    "follow mode would render an empty panel for {chord}"
                );
            }
        }
    }

    #[test]
    fn test_results_come_back_in_catalog_order() {
        let c = Chord::new(Note::new(NoteLetter::C, 0), Quality::Major);
        let scales = scales_containing(&c);
        let order: Vec<usize> = scales.iter().map(|s| s.scale_type as usize).collect();
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(order, sorted, "results must be grouped by catalog position");
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p chordflow_music_theory scale`
Expected: FAIL to compile, `cannot find function scales_containing`.

- [ ] **Step 3: Implement `scales_containing`**

Add at the bottom of `chordflow_music_theory/src/scale.rs`, above the `#[cfg(test)]` block:

```rust
/// Every catalog scale that contains all of `chord`'s notes, in catalog
/// order. This is the reverse of every other lookup in this module: given a
/// chord, what can be played over it?
///
/// Results are one scale per `(scale_type, root pitch class)`. `generate_all_roots`
/// returns 17 spellings rather than 12, so C# and Db both appear; the
/// spelling kept is the one whose notes carry the fewest accidentals, with
/// ties going to the flat. No ranking is applied beyond catalog order:
/// choosing the "best" scale over a chord is the player's call, not the app's.
pub fn scales_containing(chord: &Chord) -> Vec<Scale> {
    let wanted: Vec<i32> = chord.to_c_based_semitones();

    let mut best: Vec<(Scale, i32)> = Vec::new();

    for scale_type in ScaleType::iter() {
        for root in crate::note::generate_all_roots() {
            let scale = Scale::new(root, scale_type);
            let notes = scale.notes();
            let pitch_classes: Vec<i32> = notes
                .iter()
                .map(|n| n.to_semitones().rem_euclid(12))
                .collect();

            if !wanted.iter().all(|w| pitch_classes.contains(w)) {
                continue;
            }

            let cost = spelling_cost(&notes);
            let pitch_class = root.to_semitones().rem_euclid(12);

            match best
                .iter()
                .position(|(s, _)| s.scale_type == scale_type
                    && s.root.to_semitones().rem_euclid(12) == pitch_class)
            {
                Some(index) if cost < best[index].1 => best[index] = (scale, cost),
                Some(_) => {}
                None => best.push((scale, cost)),
            }
        }
    }

    best.into_iter().map(|(scale, _)| scale).collect()
}

/// Total accidentals across a scale's notes, plus one if the root is sharp.
/// The tiebreaker biases toward flat spellings, which read better for the
/// keys guitarists actually use (Bb mixolydian, not A# mixolydian).
fn spelling_cost(notes: &[Note]) -> i32 {
    let accidentals: i32 = notes.iter().map(|n| n.accidentals.abs()).sum();
    let sharp_root_penalty = i32::from(notes[0].accidentals > 0);
    accidentals * 2 + sharp_root_penalty
}
```

Add `use strum::IntoEnumIterator;` to the imports at the top of `chordflow_music_theory/src/scale.rs` (the derive already provides `iter()`, but the trait must be in scope for non-test code).

Because `ScaleType::iter()` is the outer loop and results are pushed in first-seen order, the returned vector is already in catalog order; no sort is needed.

- [ ] **Step 4: Run the tests**

Run: `cargo test -p chordflow_music_theory`
Expected: PASS. `test_never_empty_for_any_chord_the_practice_modes_can_produce` runs 136 chords through 459 scale candidates each; if it is noticeably slow, that is expected and still well under a second.

- [ ] **Step 5: Run the whole suite and check for warnings**

Run: `cargo test && cargo check`
Expected: PASS with no warnings.

- [ ] **Step 6: Commit**

```bash
git add chordflow_music_theory/src/scale.rs
git commit -m "feat(theory): look up which scales contain a given chord"
```

---

## Done criteria

All six tasks complete, and:

- `cargo test` passes across the workspace.
- `cargo check` reports no warnings.
- `just run` still starts the app and the Diatonic practice mode still cycles chords correctly (the only behavioural risk in this plan is Task 5's rewrite of `DiatonicConfig`).
- No file under `chordflow_desktop/src/ui/` has been modified.

The next project is the reference view, which consumes `ScaleType::iter`, `family`, `display_name`, `formula`, `Interval::degree_label`, `Scale::notes`, `Scale::diatonic_triads`, and `Scale::diatonic_sevenths`. Follow mode after that consumes `scales_containing`. Neither is in scope here.
