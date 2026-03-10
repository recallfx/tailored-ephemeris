//! Eclipse detection tests against known NASA eclipse dates (2024-2028).
//!
//! Verifies that detect_eclipse correctly identifies real eclipses and
//! does not produce false positives for regular new/full moons.
//!
//! Note: Penumbral lunar eclipses (node distance >12°) are intentionally
//! not detected — they are barely visible and not astrologically significant
//! enough to warrant looser thresholds that produce false positives.

use tailored_ephemeris::astrology::{detect_eclipse, EclipseType};
use tailored_ephemeris::julian::julday_greg;

/// Helper: detect eclipse type checking 6-hour intervals across the day,
/// matching the event detector's resolution. Picks the detection with
/// the tightest Sun-Moon orb (closest to exact syzygy).
fn eclipse_at(year: i32, month: u32, day: u32) -> Option<EclipseType> {
    let mut result: Option<EclipseType> = None;
    let mut best_orb = f64::MAX;
    for hour in [0.0, 6.0, 12.0, 18.0] {
        let jd = julday_greg(year, month as i32, day as i32, hour);
        if let Some(info) = detect_eclipse(jd).unwrap() {
            if info.sun_moon_orb < best_orb {
                best_orb = info.sun_moon_orb;
                result = Some(info.eclipse_type);
            }
        }
    }
    result
}

// ========================================================================
// Known solar eclipses (NASA confirmed) — must detect
// ========================================================================

#[test]
fn detects_2024_apr_08_solar_eclipse() {
    assert_eq!(eclipse_at(2024, 4, 8), Some(EclipseType::Solar));
}

#[test]
fn detects_2024_oct_02_solar_eclipse() {
    assert_eq!(eclipse_at(2024, 10, 2), Some(EclipseType::Solar));
}

#[test]
fn detects_2025_mar_29_solar_eclipse() {
    assert_eq!(eclipse_at(2025, 3, 29), Some(EclipseType::Solar));
}

#[test]
fn detects_2025_sep_21_solar_eclipse() {
    assert_eq!(eclipse_at(2025, 9, 21), Some(EclipseType::Solar));
}

#[test]
fn detects_2026_feb_17_solar_eclipse() {
    assert_eq!(eclipse_at(2026, 2, 17), Some(EclipseType::Solar));
}

#[test]
fn detects_2026_aug_12_solar_eclipse() {
    assert_eq!(eclipse_at(2026, 8, 12), Some(EclipseType::Solar));
}

#[test]
fn detects_2027_feb_06_solar_eclipse() {
    assert_eq!(eclipse_at(2027, 2, 6), Some(EclipseType::Solar));
}

#[test]
fn detects_2027_aug_02_solar_eclipse() {
    assert_eq!(eclipse_at(2027, 8, 2), Some(EclipseType::Solar));
}

#[test]
fn detects_2028_jan_26_solar_eclipse() {
    // Annular solar eclipse (new moon)
    assert_eq!(eclipse_at(2028, 1, 26), Some(EclipseType::Solar));
}

#[test]
fn detects_2028_jul_22_solar_eclipse() {
    // Total solar eclipse (new moon)
    assert_eq!(eclipse_at(2028, 7, 22), Some(EclipseType::Solar));
}

// ========================================================================
// Known lunar eclipses (NASA confirmed) — total and partial must detect
// ========================================================================

#[test]
fn detects_2024_mar_25_lunar_eclipse() {
    assert_eq!(eclipse_at(2024, 3, 25), Some(EclipseType::Lunar));
}

#[test]
fn detects_2024_sep_18_lunar_eclipse() {
    assert_eq!(eclipse_at(2024, 9, 18), Some(EclipseType::Lunar));
}

#[test]
fn detects_2025_mar_14_lunar_eclipse() {
    assert_eq!(eclipse_at(2025, 3, 14), Some(EclipseType::Lunar));
}

#[test]
fn detects_2025_sep_07_lunar_eclipse() {
    assert_eq!(eclipse_at(2025, 9, 7), Some(EclipseType::Lunar));
}

#[test]
fn detects_2026_mar_03_lunar_eclipse() {
    // Total lunar eclipse
    assert_eq!(eclipse_at(2026, 3, 3), Some(EclipseType::Lunar));
}

#[test]
fn detects_2028_jan_12_lunar_eclipse() {
    // Partial lunar eclipse (full moon)
    assert_eq!(eclipse_at(2028, 1, 12), Some(EclipseType::Lunar));
}

#[test]
fn detects_2028_jul_06_lunar_eclipse() {
    // Partial lunar eclipse (full moon)
    assert_eq!(eclipse_at(2028, 7, 6), Some(EclipseType::Lunar));
}

#[test]
fn detects_2028_dec_31_lunar_eclipse() {
    // Total lunar eclipse
    assert_eq!(eclipse_at(2028, 12, 31), Some(EclipseType::Lunar));
}

// ========================================================================
// Penumbral lunar eclipses — intentionally NOT detected (node too far,
// barely visible, not worth the false positive risk)
// ========================================================================

// 2027-02-20 penumbral lunar: node ~14° — acceptable miss
// 2027-07-18 penumbral lunar: node ~14° — acceptable miss
// 2027-08-17 penumbral lunar: node ~13° — acceptable miss
// 2026-08-28 penumbral lunar: may or may not detect depending on node distance

// ========================================================================
// False positive regression — regular new/full moons that are NOT eclipses
// ========================================================================

#[test]
fn no_false_positive_2026_mar_19_new_moon() {
    // Regular new moon in Pisces, node ~18.7° away — was a false positive with 18° threshold
    assert_eq!(eclipse_at(2026, 3, 19), None);
}

#[test]
fn no_false_positive_2026_apr_17_new_moon() {
    assert_eq!(eclipse_at(2026, 4, 17), None);
}

#[test]
fn no_false_positive_2026_may_17_new_moon() {
    assert_eq!(eclipse_at(2026, 5, 17), None);
}

#[test]
fn no_false_positive_2026_jun_15_new_moon() {
    assert_eq!(eclipse_at(2026, 6, 15), None);
}

#[test]
fn no_false_positive_2025_jan_29_new_moon() {
    assert_eq!(eclipse_at(2025, 1, 29), None);
}

#[test]
fn no_false_positive_2025_jun_25_new_moon() {
    assert_eq!(eclipse_at(2025, 6, 25), None);
}
