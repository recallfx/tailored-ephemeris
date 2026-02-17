//! Comprehensive validation of house cusps against Swiss Ephemeris reference values.
//!
//! Reference data from swetest v2.10.03 using swe_houses() with Placidus system.

use tailored_ephemeris::{calc_houses, julian};

/// Swiss Ephemeris reference data for 4 geographic locations.
/// Format: (label, year, month, day, hour_ut, lat, lon,
///          [cusps 1..12], mc, armc, vertex)
#[allow(dead_code)]
struct SeReference {
    label: &'static str,
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    lat: f64,
    lon: f64,
    cusps: [f64; 13], // index 0 unused, 1..12
    mc: f64,
    armc: f64,
    vertex: f64,
}

const REFERENCES: &[SeReference] = &[
    SeReference {
        label: "London",
        year: 2000, month: 1, day: 1, hour: 12.0,
        lat: 51.5074, lon: -0.1278,
        cusps: [
            0.0, // index 0 unused
            24.0146, 61.0130, 81.9114,     // cusps 1-3
            99.4932, 118.9135, 147.4855,    // cusps 4-6
            204.0146, 241.0130, 261.9114,   // cusps 7-9
            279.4932, 298.9135, 327.4855,   // cusps 10-12
        ],
        mc: 279.4932,
        armc: 280.3293,
        vertex: 188.3682,
    },
    SeReference {
        label: "New York",
        year: 1990, month: 1, day: 15, hour: 8.5,
        lat: 40.7128, lon: -74.006,
        cusps: [
            0.0,
            241.4327, 273.1664, 310.2772,
            346.9872, 17.4149, 41.4115,
            61.4327, 93.1664, 130.2772,
            166.9872, 197.4149, 221.4115,
        ],
        mc: 166.9872,
        armc: 168.0290,
        vertex: 105.5390,
    },
    SeReference {
        label: "Sydney",
        year: 1985, month: 7, day: 20, hour: 22.0,
        lat: -33.8688, lon: 151.2093,
        cusps: [
            0.0,
            136.3150, 179.7716, 215.1474,
            241.9287, 264.6947, 287.7138,
            316.3150, 359.7716, 35.1474,
            61.9287, 84.6947, 107.7138,
        ],
        mc: 61.9287,
        armc: 59.8311,
        vertex: 340.0688,
    },
    SeReference {
        label: "Tokyo",
        year: 1975, month: 3, day: 10, hour: 6.0,
        lat: 35.6762, lon: 139.6503,
        cusps: [
            0.0,
            136.3260, 159.2722, 186.8558,
            219.3433, 254.0621, 286.9726,
            316.3260, 339.2722, 6.8558,
            39.3433, 74.0621, 106.9726,
        ],
        mc: 39.3433,
        armc: 36.9467,
        vertex: 269.8108,
    },
];

/// Maximum acceptable error in degrees. Our VSOP87-based obliquity and
/// delta-T models differ slightly from Swiss Ephemeris, so we allow 0.1°
/// for primary angles (ASC, MC, Vertex) and 0.2° for intermediate cusps
/// which accumulate the obliquity difference through the iteration.
const PRIMARY_TOL: f64 = 0.1;
const CUSP_TOL: f64 = 0.2;

fn angle_diff(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 360.0;
    if d > 180.0 { 360.0 - d } else { d }
}

#[test]
fn test_ascendant_against_swiss_ephemeris() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        let diff = angle_diff(houses.ascendant, r.cusps[1]);
        assert!(
            diff < PRIMARY_TOL,
            "{}: ASC = {:.4}°, expected {:.4}° (diff {:.4}°)",
            r.label, houses.ascendant, r.cusps[1], diff
        );
    }
}

#[test]
fn test_mc_against_swiss_ephemeris() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        let diff = angle_diff(houses.mc, r.mc);
        assert!(
            diff < PRIMARY_TOL,
            "{}: MC = {:.4}°, expected {:.4}° (diff {:.4}°)",
            r.label, houses.mc, r.mc, diff
        );
    }
}

#[test]
fn test_vertex_against_swiss_ephemeris() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        let diff = angle_diff(houses.vertex, r.vertex);
        assert!(
            diff < PRIMARY_TOL,
            "{}: Vertex = {:.4}°, expected {:.4}° (diff {:.4}°)",
            r.label, houses.vertex, r.vertex, diff
        );
    }
}

#[test]
fn test_all_cusps_against_swiss_ephemeris() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        println!("\n=== {} ===", r.label);
        println!("{:<10} {:>10} {:>10} {:>8}", "Cusp", "Ours", "SwissEph", "Diff");

        let mut max_diff = 0.0_f64;
        for i in 1..=12 {
            let diff = angle_diff(houses.cusps[i], r.cusps[i]);
            max_diff = max_diff.max(diff);
            println!(
                "Cusp {:>2}    {:>10.4} {:>10.4} {:>8.4}{}",
                i, houses.cusps[i], r.cusps[i], diff,
                if diff > CUSP_TOL { " !!!" } else { "" }
            );
            assert!(
                diff < CUSP_TOL,
                "{}: Cusp {} = {:.4}°, expected {:.4}° (diff {:.4}°)",
                r.label, i, houses.cusps[i], r.cusps[i], diff
            );
        }
        println!("Max diff: {:.4}°", max_diff);
    }
}

#[test]
fn test_mc_in_same_hemisphere_as_armc() {
    let cases = [
        (2000, 1, 1, 12.0, 51.5074, -0.1278, "London J2000"),
        (1990, 1, 15, 8.5, 40.7128, -74.0060, "New York"),
        (1985, 7, 20, 22.0, -33.8688, 151.2093, "Sydney"),
        (1975, 3, 10, 6.0, 35.6762, 139.6503, "Tokyo"),
        (2000, 1, 1, 6.0, 51.5, -0.13, "London 06:00"),
        (2000, 1, 1, 18.0, 51.5, -0.13, "London 18:00"),
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
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        assert!(
            (houses.cusps[1] - houses.ascendant).abs() < 0.001,
            "{}: Cusp 1 ({:.4}°) != ASC ({:.4}°)",
            r.label, houses.cusps[1], houses.ascendant
        );
        assert!(
            (houses.cusps[10] - houses.mc).abs() < 0.001,
            "{}: Cusp 10 ({:.4}°) != MC ({:.4}°)",
            r.label, houses.cusps[10], houses.mc
        );
    }
}

#[test]
fn test_opposite_cusps_180_apart() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        for (a, b) in [(1, 7), (2, 8), (3, 9), (4, 10), (5, 11), (6, 12)] {
            let diff = angle_diff(houses.cusps[a], houses.cusps[b]);
            assert!(
                (diff - 180.0).abs() < 0.01,
                "{}: Cusps {} and {} are {:.2}° apart, expected 180°",
                r.label, a, b, diff
            );
        }
    }
}

#[test]
fn test_cusps_ordered_counterclockwise() {
    for r in REFERENCES {
        let jd = julian::julday(r.year, r.month, r.day, r.hour, 1);
        let houses = calc_houses(jd, r.lat, r.lon).unwrap();

        for i in 1..=12 {
            let next = if i == 12 { 1 } else { i + 1 };
            let mut span = houses.cusps[next] - houses.cusps[i];
            if span <= 0.0 {
                span += 360.0;
            }
            assert!(
                span > 5.0 && span < 80.0,
                "{}: House {} -> {} span is {:.1}° (cusps {:.2}° -> {:.2}°)",
                r.label, i, next, span, houses.cusps[i], houses.cusps[next]
            );
        }
    }
}
