//! Validation tests for ASC/MC fix: hemisphere consistency and London reference.

use tailored_ephemeris::{calc_houses, julian};

#[test]
fn test_london_asc_against_swiss_ephemeris() {
    // London, 2000-01-01 12:00 UT
    // Swiss Ephemeris reference: ASC = 24.01°, MC = 280.47°
    let jd = julian::julday(2000, 1, 1, 12.0, 1);
    let houses = calc_houses(jd, 51.5074, -0.1278).unwrap();

    assert!(
        (houses.ascendant - 24.01).abs() < 0.5,
        "ASC was {:.2}°, expected ~24.01°",
        houses.ascendant
    );
    // MC tolerance of 1.0° for VSOP87 vs Swiss Ephemeris precision differences
    assert!(
        (houses.mc - 280.47).abs() < 1.0,
        "MC was {:.2}°, expected ~280.47°",
        houses.mc
    );
}

#[test]
fn test_mc_in_same_hemisphere_as_armc() {
    // MC must be in the same half of the ecliptic as ARMC for all locations
    let cases = [
        (2000, 1, 1, 12.0, 51.5074, -0.1278, "London J2000"),
        (1990, 1, 15, 8.5, 40.7128, -74.0060, "New York"),
        (1985, 7, 20, 22.0, -33.8688, 151.2093, "Sydney"),
        (1975, 3, 10, 6.0, 35.6762, 139.6503, "Tokyo"),
        (2000, 1, 1, 6.0, 51.5, -0.13, "London 06:00"),   // ARMC Q3
        (2000, 1, 1, 18.0, 51.5, -0.13, "London 18:00"),   // ARMC Q1
    ];

    for (y, m, d, h, lat, lon, label) in cases {
        let jd = julian::julday(y, m, d, h, 1);
        let houses = calc_houses(jd, lat, lon).unwrap();

        let same_half = (houses.armc < 180.0) == (houses.mc < 180.0);
        assert!(
            same_half,
            "{}: ARMC={:.2}° and MC={:.2}° are in different hemispheres",
            label, houses.armc, houses.mc
        );
    }
}

#[test]
fn test_cusp1_equals_asc_cusp10_equals_mc() {
    let cases = [
        (2000, 1, 1, 12.0, 51.5074, -0.1278),
        (1990, 1, 15, 8.5, 40.7128, -74.0060),
        (1985, 7, 20, 22.0, -33.8688, 151.2093),
        (1975, 3, 10, 6.0, 35.6762, 139.6503),
    ];

    for (y, m, d, h, lat, lon) in cases {
        let jd = julian::julday(y, m, d, h, 1);
        let houses = calc_houses(jd, lat, lon).unwrap();

        assert!(
            (houses.cusps[1] - houses.ascendant).abs() < 0.001,
            "Cusp 1 ({:.4}°) != ASC ({:.4}°)",
            houses.cusps[1], houses.ascendant
        );
        assert!(
            (houses.cusps[10] - houses.mc).abs() < 0.001,
            "Cusp 10 ({:.4}°) != MC ({:.4}°)",
            houses.cusps[10], houses.mc
        );
    }
}

#[test]
fn test_opposite_cusps_180_apart() {
    let jd = julian::julday(2000, 1, 1, 12.0, 1);
    let houses = calc_houses(jd, 51.5074, -0.1278).unwrap();

    for (a, b) in [(1, 7), (2, 8), (3, 9), (4, 10), (5, 11), (6, 12)] {
        let diff = angle_diff(houses.cusps[a], houses.cusps[b]);
        assert!(
            (diff - 180.0).abs() < 0.01,
            "Cusps {} and {} are {:.2}° apart, expected 180°",
            a, b, diff
        );
    }
}

#[test]
fn test_vertex_in_valid_range() {
    let cases = [
        (2000, 1, 1, 12.0, 51.5074, -0.1278),
        (1990, 1, 15, 8.5, 40.7128, -74.0060),
        (1985, 7, 20, 22.0, -33.8688, 151.2093),
        (1975, 3, 10, 6.0, 35.6762, 139.6503),
    ];

    for (y, m, d, h, lat, lon) in cases {
        let jd = julian::julday(y, m, d, h, 1);
        let houses = calc_houses(jd, lat, lon).unwrap();

        assert!(
            houses.vertex >= 0.0 && houses.vertex < 360.0,
            "Vertex out of range: {:.2}°",
            houses.vertex
        );
    }
}

fn angle_diff(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 360.0;
    if d > 180.0 { 360.0 - d } else { d }
}
