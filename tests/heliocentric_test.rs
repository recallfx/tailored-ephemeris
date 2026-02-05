//! Integration tests for heliocentric planetary positions

use tailored_ephemeris::{calc_ut, calc_heliocentric_ut, julian, Planet};
use tailored_ephemeris::math::angle_diff;

#[test]
fn test_earth_heliocentric_opposite_sun() {
    // Earth's heliocentric longitude should be ~180° opposite to geocentric Sun longitude
    let test_dates = [
        (2000, 1, 1, 12.0),
        (2024, 3, 20, 12.0),  // vernal equinox
        (2024, 6, 21, 12.0),  // summer solstice
        (2024, 9, 22, 12.0),  // autumn equinox
        (2024, 12, 21, 12.0), // winter solstice
    ];

    for (year, month, day, hour) in test_dates {
        let jd = julian::julday(year, month, day, hour, 1);
        let sun_geo = calc_ut(jd, Planet::Sun, false).unwrap();
        let earth_helio = calc_heliocentric_ut(jd, Planet::Earth, false).unwrap();

        let diff = angle_diff(earth_helio.longitude, sun_geo.longitude).abs();
        let diff_from_180 = (diff - 180.0).abs();

        assert!(diff_from_180 < 1.0,
                "{}-{:02}-{:02}: Earth helio ({:.4}°) should be ~180° from Sun geo ({:.4}°), diff from 180° = {:.4}°",
                year, month, day, earth_helio.longitude, sun_geo.longitude, diff_from_180);
    }
}

#[test]
fn test_earth_distance_range() {
    // Earth's distance from Sun: perihelion ~0.983 AU, aphelion ~1.017 AU
    let test_jds: Vec<f64> = (0..12).map(|m| {
        julian::julday(2024, m + 1, 15, 12.0, 1)
    }).collect();

    for jd in test_jds {
        let earth = calc_heliocentric_ut(jd, Planet::Earth, false).unwrap();
        assert!(earth.distance > 0.983 && earth.distance < 1.017,
                "Earth distance {:.6} AU out of expected range at JD {:.1}", earth.distance, jd);
    }
}

#[test]
fn test_mercury_distance_range() {
    // Mercury heliocentric distance: ~0.307 - 0.467 AU
    let jd = julian::julday(2024, 1, 1, 12.0, 1);
    let pos = calc_heliocentric_ut(jd, Planet::Mercury, false).unwrap();
    assert!(pos.distance > 0.3 && pos.distance < 0.47,
            "Mercury helio distance {:.6} AU out of range", pos.distance);
}

#[test]
fn test_jupiter_distance_range() {
    // Jupiter heliocentric distance: ~4.95 - 5.46 AU
    let jd = julian::julday(2024, 1, 1, 12.0, 1);
    let pos = calc_heliocentric_ut(jd, Planet::Jupiter, false).unwrap();
    assert!(pos.distance > 4.9 && pos.distance < 5.5,
            "Jupiter helio distance {:.6} AU out of range", pos.distance);
}

#[test]
fn test_all_speeds_positive() {
    // All heliocentric speeds should be positive (planets always move prograde
    // in heliocentric frame)
    let jd = julian::julday(2024, 1, 1, 12.0, 1);

    for &planet in Planet::heliocentric_planets() {
        let pos = calc_heliocentric_ut(jd, planet, true).unwrap();
        assert!(pos.speed_longitude > 0.0,
                "{:?} heliocentric speed should be positive, got {:.6}°/day",
                planet, pos.speed_longitude);
    }
}

#[test]
fn test_outer_planets_slower_than_inner() {
    let jd = julian::julday(2024, 1, 1, 12.0, 1);

    let mercury = calc_heliocentric_ut(jd, Planet::Mercury, true).unwrap();
    let venus = calc_heliocentric_ut(jd, Planet::Venus, true).unwrap();
    let earth = calc_heliocentric_ut(jd, Planet::Earth, true).unwrap();
    let mars = calc_heliocentric_ut(jd, Planet::Mars, true).unwrap();
    let jupiter = calc_heliocentric_ut(jd, Planet::Jupiter, true).unwrap();
    let saturn = calc_heliocentric_ut(jd, Planet::Saturn, true).unwrap();
    let uranus = calc_heliocentric_ut(jd, Planet::Uranus, true).unwrap();
    let neptune = calc_heliocentric_ut(jd, Planet::Neptune, true).unwrap();
    let pluto = calc_heliocentric_ut(jd, Planet::Pluto, true).unwrap();

    assert!(mercury.speed_longitude > venus.speed_longitude,
            "Mercury ({:.4}) should be faster than Venus ({:.4})",
            mercury.speed_longitude, venus.speed_longitude);
    assert!(venus.speed_longitude > earth.speed_longitude,
            "Venus ({:.4}) should be faster than Earth ({:.4})",
            venus.speed_longitude, earth.speed_longitude);
    assert!(earth.speed_longitude > mars.speed_longitude,
            "Earth ({:.4}) should be faster than Mars ({:.4})",
            earth.speed_longitude, mars.speed_longitude);
    assert!(mars.speed_longitude > jupiter.speed_longitude,
            "Mars ({:.4}) should be faster than Jupiter ({:.4})",
            mars.speed_longitude, jupiter.speed_longitude);
    assert!(jupiter.speed_longitude > saturn.speed_longitude,
            "Jupiter ({:.4}) should be faster than Saturn ({:.4})",
            jupiter.speed_longitude, saturn.speed_longitude);
    assert!(saturn.speed_longitude > uranus.speed_longitude,
            "Saturn ({:.4}) should be faster than Uranus ({:.4})",
            saturn.speed_longitude, uranus.speed_longitude);
    assert!(uranus.speed_longitude > neptune.speed_longitude,
            "Uranus ({:.4}) should be faster than Neptune ({:.4})",
            uranus.speed_longitude, neptune.speed_longitude);
    assert!(neptune.speed_longitude > pluto.speed_longitude,
            "Neptune ({:.4}) should be faster than Pluto ({:.4})",
            neptune.speed_longitude, pluto.speed_longitude);
}

#[test]
fn test_heliocentric_mars_cross_validation() {
    // Cross-validate: geocentric Mars longitude should differ from heliocentric
    // by an amount related to Earth's position. The geocentric position is the
    // heliocentric position shifted by Earth's orbital position (parallax effect).
    let jd = julian::julday(2024, 1, 1, 12.0, 1);

    let mars_geo = calc_ut(jd, Planet::Mars, false).unwrap();
    let mars_helio = calc_heliocentric_ut(jd, Planet::Mars, false).unwrap();
    let earth_helio = calc_heliocentric_ut(jd, Planet::Earth, false).unwrap();

    // The difference between geocentric and heliocentric should not be extreme
    // For Mars it can be up to ~40° depending on relative positions
    let diff = angle_diff(mars_geo.longitude, mars_helio.longitude).abs();
    assert!(diff < 50.0,
            "Mars geo-helio difference should be reasonable, got {:.4}°", diff);

    // Print for diagnostic purposes
    println!("Mars geocentric:   {:.4}°", mars_geo.longitude);
    println!("Mars heliocentric: {:.4}°", mars_helio.longitude);
    println!("Earth helio:       {:.4}°", earth_helio.longitude);
    println!("Geo-helio diff:    {:.4}°", diff);
}

#[test]
fn test_invalid_planets_for_heliocentric() {
    // Sun, Moon, and TrueNode should return errors for heliocentric
    let jd = julian::julday(2024, 1, 1, 12.0, 1);

    assert!(calc_heliocentric_ut(jd, Planet::Sun, false).is_err(),
            "Sun should not be valid for heliocentric");
    assert!(calc_heliocentric_ut(jd, Planet::Moon, false).is_err(),
            "Moon should not be valid for heliocentric");
    assert!(calc_heliocentric_ut(jd, Planet::TrueNode, false).is_err(),
            "TrueNode should not be valid for heliocentric");
}

#[test]
fn test_heliocentric_earth_zero_latitude() {
    // Earth orbits in the ecliptic plane, so latitude should be 0
    let jd = julian::julday(2024, 6, 15, 12.0, 1);
    let earth = calc_heliocentric_ut(jd, Planet::Earth, false).unwrap();
    assert!(earth.latitude.abs() < 0.001,
            "Earth heliocentric latitude should be ~0, got {:.6}°", earth.latitude);
}

#[test]
fn test_heliocentric_positions_count() {
    // get_all_heliocentric_positions should return 9 planets
    let jd = julian::julday(2024, 1, 1, 12.0, 1);
    let positions = tailored_ephemeris::astrology::get_all_heliocentric_positions(jd).unwrap();
    assert_eq!(positions.len(), 9, "Should have 9 heliocentric planets");

    // First should be Earth
    assert_eq!(positions[0].planet_key, "earth");

    // None should be retrograde
    for p in &positions {
        assert!(!p.is_retrograde, "{} should not be retrograde in heliocentric", p.planet_key);
    }

    // Check all expected planets are present
    let keys: Vec<&str> = positions.iter().map(|p| p.planet_key).collect();
    assert!(keys.contains(&"earth"));
    assert!(keys.contains(&"mercury"));
    assert!(keys.contains(&"venus"));
    assert!(keys.contains(&"mars"));
    assert!(keys.contains(&"jupiter"));
    assert!(keys.contains(&"saturn"));
    assert!(keys.contains(&"uranus"));
    assert!(keys.contains(&"neptune"));
    assert!(keys.contains(&"pluto"));
}
