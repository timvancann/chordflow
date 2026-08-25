use super::{interval::Interval, quality::Quality};

const NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];

/// The roman numeral for a scale degree, cased by the chord's third: `ii` for
/// a minor-ish chord, `V` for a major-ish one, as it is conventionally written.
///
/// `degree` is zero-based, so degree 4 of a major scale is `V`.
///
/// The case is derived from the quality's intervals rather than a hand-written
/// list of "minor" qualities, so a quality added to the crate is cased right
/// without anyone remembering this function exists.
pub fn roman_numeral(degree: usize, quality: Quality) -> Option<String> {
    let numeral = NUMERALS.get(degree)?;

    Some(if has_minor_third(quality) {
        numeral.to_lowercase()
    } else {
        numeral.to_string()
    })
}

fn has_minor_third(quality: Quality) -> bool {
    quality.to_intervals().contains(&Interval::MinorThird)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale_degrees_read_the_conventional_way() {
        let degrees: Vec<String> = [
            Quality::MajorSeventh,
            Quality::MinorSeventh,
            Quality::MinorSeventh,
            Quality::MajorSeventh,
            Quality::Dominant,
            Quality::MinorSeventh,
            Quality::HalfDiminished,
        ]
        .into_iter()
        .enumerate()
        .map(|(degree, quality)| roman_numeral(degree, quality).unwrap())
        .collect();

        assert_eq!(
            degrees,
            vec!["I", "ii", "iii", "IV", "V", "vi", "vii"],
            "the major scale's sevenths"
        );
    }

    #[test]
    fn test_case_follows_the_third_for_every_quality() {
        let minor_ish = [
            Quality::Minor,
            Quality::Diminished,
            Quality::MinorSeventh,
            Quality::HalfDiminished,
            Quality::DiminishedSeventh,
            Quality::MinorMajorSeventh,
        ];
        for quality in minor_ish {
            assert_eq!(
                roman_numeral(0, quality).unwrap(),
                "i",
                "{quality:?} has a minor third"
            );
        }

        let major_ish = [
            Quality::Major,
            Quality::Augmented,
            Quality::Dominant,
            Quality::MajorSeventh,
            Quality::AugmentedMajorSeventh,
        ];
        for quality in major_ish {
            assert_eq!(
                roman_numeral(0, quality).unwrap(),
                "I",
                "{quality:?} has a major third"
            );
        }
    }

    #[test]
    fn test_out_of_range_degrees_have_no_numeral() {
        assert_eq!(roman_numeral(7, Quality::Major), None);
        assert_eq!(roman_numeral(99, Quality::Major), None);
    }
}
