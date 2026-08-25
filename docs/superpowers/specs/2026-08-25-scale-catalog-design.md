# Scale catalog in chordflow_music_theory

Date: 2026-08-25
Status: approved, ready for planning

## Context

ChordFlow should grow an in-app reference page: a digital version of the
author's own `Scales.pdf` poster, showing scale formulas grouped by family,
the notes those formulas produce in a chosen key, and the chords that fit
each scale. The reference view gains a "follow" mode that tracks the live
practice state and answers the improviser's question: a chord just came up,
which scales contain it?

The whole idea is too large for one spec. It decomposes into five projects,
each with its own spec, plan and tests:

1. **Scale catalog** (this document). Pure logic in `chordflow_music_theory`.
2. **Reference view.** Full-screen browse surface: key selector, family
   tables, notes in key, chords in scale.
3. **Follow mode.** Chord-follow wired to live practice state.
4. **Detachable window.** The reference view as its own desktop window.
5. **Fretboard diagram.** Root-relative degree-dot fretboard, as on the poster.

This spec covers project 1 only. It adds no UI and changes no user-visible
behaviour. Its deliverable is a tested theory layer that projects 2 and 3
can consume.

## Goals

- Represent all 27 scales from the poster as degree formulas.
- Spell the notes of any of those scales in any key, correctly, including
  double accidentals.
- Derive the triads and seventh chords that fit a scale, for the scales
  where that is defined.
- Look up, for a given chord, every scale that contains it.

## Non-goals

- Any UI, component, or view. No file under `chordflow_desktop/src/ui/`
  is touched.
- Chord theory beyond "these chords fit this scale". Voicings, extensions,
  substitutions, and voice leading are out of scope, permanently.
- Fretboard geometry or rendering.
- Scale-degree analysis of arbitrary progressions (key inference).

## The catalog

Formulas are transcribed from `Scales.pdf`, which is the authority. Each
scale belongs to one of four families, matching the poster's grouping.

### Major

| Scale | Formula |
|---|---|
| ionian | R 2 3 4 5 6 7 |
| dorian | R 2 b3 4 5 6 b7 |
| phrygian | R b2 b3 4 5 b6 b7 |
| lydian | R 2 3 #4 5 6 7 |
| mixolydian | R 2 3 4 5 6 b7 |
| aeolian | R 2 b3 4 5 b6 b7 |
| locrian | R b2 b3 4 b5 b6 b7 |

### Melodic minor

| Scale | Formula |
|---|---|
| melodic minor | R 2 b3 4 5 6 7 |
| dorian b2 | R b2 b3 4 5 6 b7 |
| lydian augmented | R 2 3 #4 #5 6 7 |
| lydian dominant | R 2 3 #4 5 6 b7 |
| mixolydian b6 | R 2 3 4 5 b6 b7 |
| aeolian b5 | R 2 b3 4 b5 b6 b7 |
| altered | R b2 #2 3 b5 #5 b7 |

### Harmonic minor

| Scale | Formula |
|---|---|
| harmonic minor | R 2 b3 4 5 b6 7 |
| locrian natural 6 | R b2 b3 4 b5 6 b7 |
| ionian augmented | R 2 3 4 #5 6 7 |
| locrian #4 | R 2 b3 #4 5 6 b7 |
| phrygian dominant | R b2 3 4 5 b6 b7 |
| lydian #9 | R #2 3 #4 5 6 7 |
| superlocrian bb7 | R b2 b3 b4 b5 b6 bb7 |

The poster names the fourth entry "locrian #4"; that name is kept verbatim
even though the formula is more commonly called dorian #4 or Ukrainian
dorian. The poster is the authority for names as well as formulas.

### Other

| Scale | Formula | Note count |
|---|---|---|
| major blues | R 2 b3 3 5 6 | 6 |
| minor blues | R b3 4 b5 5 b7 | 6 |
| whole tone | R 2 3 #4 #5 b7 | 6 |
| augmented | R #2 3 5 #5 7 | 6 |
| diminished half whole | R b2 #2 3 b5 5 6 b7 | 8 |
| diminished whole half | R 2 b3 4 b5 #5 6 7 | 8 |

**Known deviation from the poster.** The poster's major blues row renders as
`R 2 b3 b3 5 6`, repeating the flat third. This spec uses `R 2 b3 3 5 6`
(major pentatonic plus the flat third), the standard formula. If the poster
is correct as printed, this row changes and the corresponding test changes
with it.

The six scales in this family are the non-heptatonic ones. They are the
reason the chords-in-scale functions return an `Option`.

## Design

### Interval gains four spellings

`Interval` (`chordflow_music_theory/src/interval.rs`) cannot currently
express `#2`, `b4`, `#5`, or `bb7`, all of which the catalog needs. Four
variants are added, each defined by its semitone count and its letter-step
(the value `to_index` returns, which is what drives correct spelling in
`Note::add_interval`):

| Variant | Semitones | Letter-step |
|---|---|---|
| `AugmentedSecond` | 3 | 1 |
| `DiminishedFourth` | 4 | 3 |
| `AugmentedFifth` | 8 | 4 |
| `DiminishedSeventh` | 9 | 6 |

Existing variants keep their semitone values, so nothing currently working
changes behaviour. `from_semitone` stays lossy (it collapses `#4`/`b5` to
`Tritone`) and is left alone: its only consumers are chord-quality paths
where spelling does not matter, and the catalog never calls it.

The new variants are inserted in musical order rather than appended.
`Interval::from_repr` is not used anywhere in the workspace, and
`Interval::iter()` has exactly one consumer, the `test_add_interval` test in
`note.rs`, whose expected-note list is extended to match. That test then
also covers the new spellings.

`Interval` gains `degree_label(self) -> &'static str`, returning the poster's
notation (`"R"`, `"b2"`, `"#4"`, `"bb7"`). This is notation, not
presentation, so it belongs in the theory crate; it is what lets the tests
read directly against the poster and what project 2 renders in the tables.

### ScaleType becomes the catalog

`ScaleType` currently has one variant, `Diatonic`, and `Scale::new` matches
on it to produce a hardcoded interval vector (`scale.rs:29`). It is replaced
by the 27 variants above, and gains:

- `family(self) -> ScaleFamily` where `ScaleFamily` is
  `Major | MelodicMinor | HarmonicMinor | Other`.
- `formula(self) -> Vec<Interval>`.
- `display_name(self) -> &'static str`, the poster's name.

`Scale::new` reduces to reading `scale_type.formula()`. `Scale` keeps its
current shape (`root`, `scale_type`, `intervals`).

`Diatonic` is renamed `Ionian`. Call sites: `diatonic.rs:20`, `diatonic.rs:56`,
`diatonic.rs:58`, `mode.rs:45`, and four occurrences in `practice.rs` tests.
The rename is mechanical and is the only change to `chordflow_desktop`.

### Quality gains three variants and stops panicking

Stacking thirds through 21 heptatonic scales produces qualities `Quality`
cannot name. Three variants are added:

| Variant | Semitones | Symbol | Name |
|---|---|---|---|
| `DiminishedSeventh` | 0 3 6 9 | `o7` | Diminished Seventh |
| `MinorMajorSeventh` | 0 3 7 11 | `-Δ` | Minor Major Seventh |
| `AugmentedMajorSeventh` | 0 4 8 11 | `+Δ` | Augmented Major Seventh |

`from_string`, `from_name`, `name`, `to_intervals` and `from_intervals` are
extended for all three.

`Quality::from_intervals` has a bug: it maps `[0, 5, 7]` to `Augmented`
(`quality.rs:81`), while `to_intervals` defines augmented as `[0, 4, 8]`.
`[0, 5, 7]` is a suspended fourth, not an augmented triad. The mapping is
corrected to `[0, 4, 8] => Augmented`. This is latent in the current app
because the major scale never yields an augmented triad, but harmonic minor's
third degree does, so the catalog would hit it immediately.

`from_intervals` also panics on any unrecognized interval set
(`quality.rs:97`). Its signature changes to
`from_intervals(intervals: Vec<i32>) -> Option<Quality>`. Its one existing
caller, `calculate_chord_quality_in_scale` in
`chordflow_desktop/src/state/diatonic.rs:88`, works only on the major scale
and can `.expect()` with a message naming the unrecognized set; this keeps
current behaviour while removing the silent-panic surface. `[0, 5, 7]` is
deliberately left unmapped rather than given a `Sus4` variant: suspended
chords are not produced by stacking thirds, and adding a quality with no
producer would be speculative.

### Three new functions

**`Scale::notes(&self) -> Vec<Note>`**

Maps the formula over `self.root.add_interval`. This is the capability paper
cannot have: the same formula respelled per key. G lydian is
`G A B C# D E F#`; F# ionian genuinely contains `E#`; G superlocrian bb7
contains `Fb`.

**`Scale::diatonic_triads(&self) -> Option<Vec<Chord>>`** and
**`Scale::diatonic_sevenths(&self) -> Option<Vec<Chord>>`**

Both generalize `calculate_chord_quality_in_scale`
(`chordflow_desktop/src/state/diatonic.rs:88`), which currently hardcodes the
7-note assumption via `normalize(i, 7)`. For each degree, take scale indices
`i, i+2, i+4` (triads) or `i, i+2, i+4, i+6` (sevenths) modulo the scale
length, reduce to root-relative semitones, and name the result via
`Quality::from_intervals`. They share one private helper parameterised by the
index offsets; the two public functions exist because the reference view
shows them as two separate rows and because triads alone are what the
existing Diatonic practice mode needs.

Both return `None` for the six non-heptatonic scales, where stacking by scale
index has no accepted meaning. Degrees whose interval set has no matching
`Quality` are skipped rather than mislabelled, so a returned vector may be
shorter than the scale. This is a deliberate honesty-over-completeness
choice: a wrong chord symbol on a reference page is worse than a missing one.
It is also what keeps the exotic scales usable at all, since scales like
superlocrian bb7 produce several sets that no conventional symbol names.

The generalized logic moves into `chordflow_music_theory`;
`calculate_chord_quality_in_scale` in the desktop crate becomes a caller
rather than an owner of the algorithm.

**`scales_containing(chord: &Chord) -> Vec<Scale>`** (free function in `scale.rs`)

The follow-mode lookup, running the opposite direction from everything above.
Builds every scale in the catalog across every root from
`note::generate_all_roots()`, and keeps those whose pitch-class set is a
superset of `chord.to_c_based_semitones()`.

`generate_all_roots()` returns 17 spellings, not 12: seven letters times
three accidentals, minus the four it filters out (`note.rs:159` drops Cb, E#,
B#, Fb). So C# and Db are both present and would surface the same scale twice
under two names. Matches are therefore deduplicated by
`(scale_type, root pitch class)`, keeping the spelling whose `notes()` carry
the fewest total accidentals, with ties broken toward the flat spelling. That
yields Bb mixolydian rather than A# mixolydian, and at most 27 times 12 is
324 results, cheap enough to compute on demand with no caching.

Results are returned in catalog order (family, then position within family),
so the consuming view gets a stable, groupable list without sorting.

## Testing

`Scales.pdf` is the oracle. The test suite is table-driven, one row per scale.

- **Formula coverage.** For each of the 27 scales, assert
  `formula().map(degree_label)` equals the poster's row verbatim. This makes
  the poster mechanically checkable and makes any future transcription error
  a test failure.
- **Spelling.** For each scale, assert `notes()` in a chosen key against
  hand-computed spellings. Explicit cases for the hard ones: F# ionian
  contains `E#`, C superlocrian bb7 contains `Bbb`, G lydian contains `C#`,
  Db aeolian b5 spells its `b5` as `Abb`.
- **Chords in scale.** Assert `diatonic_triads()` for the well-known cases:
  C ionian gives `C Dm Em F G Am Bdim`, and C harmonic minor's third degree
  is augmented (the case that panics under the current `from_intervals`).
  Assert `diatonic_sevenths()` gives C ionian `Cmaj7 Dm7 Em7 Fmaj7 G7 Am7
  Bm7b5`, C melodic minor's first degree minor-major seventh, C harmonic
  minor's seventh degree diminished seventh, and C lydian augmented's first
  degree augmented-major seventh. These four cases are the only producers of
  the three new `Quality` variants, so they are what justifies adding them.
  Assert `None` from both functions for all six non-heptatonic scales.
- **Reverse lookup.** Assert `scales_containing` finds the obvious members
  (Cmaj7 is in C ionian, C lydian, and G ionian) and excludes obvious
  non-members (Cmaj7 is not in C phrygian). Assert the result is non-empty
  for every `(root, quality)` pair the existing practice modes can generate,
  so follow mode can never render an empty panel in normal use. Assert the
  deduplication rule: a chord matching a scale rooted on the C#/Db pitch
  class returns that scale exactly once, spelled the way the rule prescribes.
- **Regression.** The existing tests in `scale.rs`, `note.rs` and
  `practice.rs` continue to pass after the `Diatonic` to `Ionian` rename and
  the `Interval` additions.

## Out of scope for this project, by design

`scales_containing` returns scales, not a ranking. Deciding which scale is
the "best" choice over a chord is a musical judgement the app should not
make; the reference view presents the options and the player chooses. If
ordering by consonance ever becomes desirable, it is a later change with its
own argument.
