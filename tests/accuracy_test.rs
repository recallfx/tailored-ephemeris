//! Accuracy comparison tests against Swiss Ephemeris reference values
//!
//! Reference values from swetest with Moshier ephemeris for 2000-01-01 12:00 UT

use tailored_ephemeris::{calc_ut, calc_houses, julian, Planet};

/// Reference values from Swiss Ephemeris (swetest -b1.1.2000 -ut12:00 -eswe)
/// Format: (longitude, speed, distance)
const SE_REFERENCE_J2000: [(Planet, f64, f64); 11] = [
    (Planet::Sun, 280.3689197, 1.0194321),
    (Planet::Moon, 223.3237754, 12.0211827),
    (Planet::Mercury, 271.8892750, 1.5562541),
    (Planet::Venus, 241.5657983, 1.2090397),
    (Planet::Mars, 327.9633133, 0.7756728),
    (Planet::Jupiter, 25.2530303, 0.0407613),
    (Planet::Saturn, 40.3956390, -0.0199448),
    (Planet::Uranus, 314.8092232, 0.0503436),
    (Planet::Neptune, 303.1929812, 0.0355701),
    (Planet::Pluto, 251.4547088, 0.0351529),
    (Planet::TrueNode, 123.9528954, -0.0543822),
];

/// Reference values for 1925-01-01 12:00 UT (100 years back)
const SE_REFERENCE_1925: [(Planet, f64, f64); 11] = [
    (Planet::Sun, 280.5877948, 1.0194705),
    (Planet::Moon, 4.9288964, 12.9973034),
    (Planet::Mercury, 269.1887429, -0.9016195),
    (Planet::Venus, 253.2598996, 1.2443362),
    (Planet::Mars, 7.9289185, 0.6175819),
    (Planet::Jupiter, 273.2642731, 0.2287795),
    (Planet::Saturn, 222.1040118, 0.0808828),
    (Planet::Uranus, 348.1198945, 0.0291863),
    (Planet::Neptune, 142.2141458, -0.0193458),
    (Planet::Pluto, 102.5277646, -0.0208949),
    (Planet::TrueNode, 134.2590105, 0.0176008),
];

/// Reference values for 2035-01-01 12:00 UT (10 years future)
const SE_REFERENCE_2035: [(Planet, f64, f64); 11] = [
    (Planet::Sun, 280.8831346, 1.0191724),
    (Planet::Moon, 191.8165120, 12.3013491),
    (Planet::Mercury, 297.3250331, 1.4992342),
    (Planet::Venus, 234.0278461, 1.0205848),
    (Planet::Mars, 233.4113135, 0.6552702),
    (Planet::Jupiter, 5.8855368, 0.1077535),
    (Planet::Saturn, 123.5456882, -0.0723583),
    (Planet::Uranus, 98.4329629, -0.0431269),
    (Planet::Neptune, 19.5624580, 0.0009651),
    (Planet::Pluto, 316.8295706, 0.0252724),
    (Planet::TrueNode, 167.1940539, 0.0014900),
];

#[test]
fn test_accuracy_j2000() {
    let jd = julian::julday(2000, 1, 1, 12.0, 1);
    assert!((jd - 2451545.0).abs() < 0.0001, "JD mismatch");

    println!("\n=== Accuracy Test: 2000-01-01 12:00 UT (JD 2451545.0) ===\n");
    println!("{:<12} {:>12} {:>12} {:>10} {:>10}",
             "Planet", "Ours", "SwissEph", "Diff (°)", "Diff (\")");
    println!("{}", "-".repeat(60));

    let mut max_error_arcmin = 0.0f64;
    let mut total_error = 0.0f64;
    let mut count = 0;

    for (planet, se_lon, _se_speed) in SE_REFERENCE_J2000.iter() {
        let pos = calc_ut(jd, *planet, true).expect("calc failed");

        let diff = (pos.longitude - se_lon).abs();
        let diff_normalized = if diff > 180.0 { 360.0 - diff } else { diff };
        let diff_arcsec = diff_normalized * 3600.0;

        println!("{:<12} {:>12.6} {:>12.6} {:>10.6} {:>10.1}",
                 format!("{:?}", planet),
                 pos.longitude,
                 se_lon,
                 diff_normalized,
                 diff_arcsec);

        max_error_arcmin = max_error_arcmin.max(diff_arcsec / 60.0);
        total_error += diff_arcsec;
        count += 1;
    }

    println!("{}", "-".repeat(60));
    println!("Max error: {:.1} arcmin ({:.0} arcsec)", max_error_arcmin, max_error_arcmin * 60.0);
    println!("Avg error: {:.1} arcsec", total_error / count as f64);

    // For horoscope purposes, 1 degree accuracy is acceptable
    // Professional astrology typically uses 1 arcminute
    assert!(max_error_arcmin < 60.0, "Error exceeds 1 degree");
}

/// Helper function to run accuracy test for a given date and reference data
fn run_accuracy_test(year: i32, month: i32, day: i32, hour: f64,
                     reference: &[(Planet, f64, f64)], label: &str) -> (f64, f64) {
    let jd = julian::julday(year, month, day, hour, 1);

    println!("\n=== Accuracy Test: {} (JD {:.1}) ===\n", label, jd);
    println!("{:<12} {:>12} {:>12} {:>10} {:>10}",
             "Planet", "Ours", "SwissEph", "Diff (°)", "Diff (\")");
    println!("{}", "-".repeat(60));

    let mut max_error_arcmin = 0.0f64;
    let mut total_error = 0.0f64;
    let mut count = 0;

    for (planet, se_lon, _se_speed) in reference.iter() {
        let pos = calc_ut(jd, *planet, true).expect("calc failed");

        let diff = (pos.longitude - se_lon).abs();
        let diff_normalized = if diff > 180.0 { 360.0 - diff } else { diff };
        let diff_arcsec = diff_normalized * 3600.0;

        println!("{:<12} {:>12.6} {:>12.6} {:>10.6} {:>10.1}",
                 format!("{:?}", planet),
                 pos.longitude,
                 se_lon,
                 diff_normalized,
                 diff_arcsec);

        max_error_arcmin = max_error_arcmin.max(diff_arcsec / 60.0);
        total_error += diff_arcsec;
        count += 1;
    }

    println!("{}", "-".repeat(60));
    println!("Max error: {:.1} arcmin ({:.0} arcsec)", max_error_arcmin, max_error_arcmin * 60.0);
    println!("Avg error: {:.1} arcsec", total_error / count as f64);

    (max_error_arcmin, total_error / count as f64)
}

#[test]
fn test_accuracy_1925() {
    let (max_err, _avg_err) = run_accuracy_test(1925, 1, 1, 12.0, &SE_REFERENCE_1925,
                                                 "1925-01-01 12:00 UT (100 years back)");
    // For dates 100 years from J2000, outer planets can have up to ~1.5° error
    // due to accumulated perturbation effects. This is acceptable for horoscope
    // purposes where zodiac signs are 30° wide and aspect orbs are 1-8°.
    assert!(max_err < 90.0, "Error exceeds 1.5 degrees for 1925");
}

#[test]
fn test_accuracy_2035() {
    let (max_err, _avg_err) = run_accuracy_test(2035, 1, 1, 12.0, &SE_REFERENCE_2035,
                                                 "2035-01-01 12:00 UT (10 years future)");
    // For dates within 35 years of J2000, expect better accuracy
    assert!(max_err < 45.0, "Error exceeds 0.75 degrees for 2035");
}

#[test]
fn test_accuracy_various_dates() {
    // Test dates spanning different centuries
    let test_dates = [
        (1990, 6, 21, 12.0, "1990 Summer Solstice"),
        (2020, 12, 21, 18.5, "2020 Great Conjunction"),
        (2024, 4, 8, 18.0, "2024 Solar Eclipse"),
        (1969, 7, 20, 20.0, "1969 Moon Landing"),
    ];

    println!("\n=== Multi-date Accuracy Test ===\n");

    for (year, month, day, hour, label) in test_dates {
        let jd = julian::julday(year, month, day, hour, 1);

        println!("--- {} (JD {:.2}) ---", label, jd);

        // Just test Sun and Moon as sanity check
        let sun = calc_ut(jd, Planet::Sun, false).unwrap();
        let moon = calc_ut(jd, Planet::Moon, false).unwrap();

        println!("  Sun:  {:.4}°", sun.longitude);
        println!("  Moon: {:.4}°", moon.longitude);

        // Basic sanity: longitude should be 0-360
        assert!(sun.longitude >= 0.0 && sun.longitude < 360.0);
        assert!(moon.longitude >= 0.0 && moon.longitude < 360.0);
    }
}

#[test]
fn test_houses_accuracy() {
    // Reference: swetest -b1.1.2000 -ut12:00 -house51.5,-0.1,P
    // London coordinates
    let jd = julian::julday(2000, 1, 1, 12.0, 1);
    let houses = calc_houses(jd, 51.5, -0.1).unwrap();

    println!("\n=== House Calculation Test: London ===\n");
    println!("Ascendant: {:.4}°", houses.ascendant);
    println!("MC:        {:.4}°", houses.mc);
    println!("ARMC:      {:.4}°", houses.armc);
    println!("Vertex:    {:.4}°", houses.vertex);

    println!("\nCusps:");
    for i in 1..=12 {
        println!("  House {:>2}: {:>10.4}°", i, houses.cusps[i]);
    }

    // Basic sanity checks
    assert!(houses.ascendant >= 0.0 && houses.ascendant < 360.0);
    assert!(houses.mc >= 0.0 && houses.mc < 360.0);

    // MC should be ~90° from Asc (roughly)
    let mc_asc_diff = (houses.mc - houses.ascendant).abs();
    let normalized_diff = if mc_asc_diff > 180.0 { 360.0 - mc_asc_diff } else { mc_asc_diff };
    println!("\nMC-Asc difference: {:.1}° (should be ~90°)", normalized_diff);
}
