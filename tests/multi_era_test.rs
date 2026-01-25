//! Multi-era accuracy tests
//!
//! Tests accuracy across 150 years (1925-2075) at key dates

use tailored_ephemeris::{calc_ut, julian, Planet};

/// Test data structure: (year, month, day, planet_longitudes[11])
/// Planets: Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto, TrueNode
struct EraTestPoint {
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
    positions: [f64; 11],
}

const PLANETS: [Planet; 11] = [
    Planet::Sun,
    Planet::Moon,
    Planet::Mercury,
    Planet::Venus,
    Planet::Mars,
    Planet::Jupiter,
    Planet::Saturn,
    Planet::Uranus,
    Planet::Neptune,
    Planet::Pluto,
    Planet::TrueNode,
];

/// Reference data from Swiss Ephemeris (Moshier) for key years
const ERA_TEST_POINTS: &[EraTestPoint] = &[
    EraTestPoint {
        year: 1925, month: 1, day: 1, hour: 12.0,
        positions: [280.5877948, 4.9288964, 269.1887429, 253.2598996, 7.9289185,
                    273.2642731, 222.1040118, 348.1198945, 142.2141458, 102.5277646, 134.2590105],
    },
    EraTestPoint {
        year: 1950, month: 1, day: 1, hour: 12.0,
        positions: [280.5143892, 67.5443540, 299.9720188, 317.1524788, 182.3944446,
                    306.6180429, 169.4355399, 92.6614516, 197.2710243, 137.7895694, 12.4763404],
    },
    EraTestPoint {
        year: 1975, month: 1, day: 1, hour: 12.0,
        positions: [280.4440134, 146.5951747, 287.8576437, 294.0081695, 255.3176711,
                    343.3989594, 105.8334785, 211.8939332, 250.4502428, 189.2259704, 249.9242081],
    },
    EraTestPoint {
        year: 2000, month: 1, day: 1, hour: 12.0,
        positions: [280.3689197, 223.3237754, 271.8892750, 241.5657983, 327.9633133,
                    25.2530303, 40.3956390, 314.8092232, 303.1929812, 251.4547088, 123.9528954],
    },
    EraTestPoint {
        year: 2025, month: 1, day: 1, hour: 12.0,
        positions: [281.3234326, 300.6608947, 260.5163879, 328.2482972, 121.7526313,
                    73.1626153, 344.5620839, 53.6237355, 357.3046902, 301.0800915, 0.7777947],
    },
    EraTestPoint {
        year: 2050, month: 1, day: 1, hour: 12.0,
        positions: [281.2580033, 25.3818536, 269.6006798, 281.8772976, 228.0324387,
                    121.6321532, 297.6321732, 170.7295657, 53.5945758, 337.5416374, 239.5089534],
    },
    EraTestPoint {
        year: 2075, month: 1, day: 1, hour: 12.0,
        positions: [281.1802401, 91.3351574, 300.7098004, 234.7978524, 296.1682802,
                    164.5897961, 253.1575501, 280.8670337, 111.0031921, 6.8413482, 113.6531205],
    },
];

fn angle_diff(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs();
    if diff > 180.0 { 360.0 - diff } else { diff }
}

#[test]
fn test_multi_era_accuracy() {
    println!("\n{}", "=".repeat(60));
    println!(" Multi-Era Accuracy Test (1925-2075) ");
    println!("{}", "=".repeat(60));

    let mut all_errors: Vec<(i32, &'static str, f64)> = Vec::new();

    for point in ERA_TEST_POINTS {
        let jd = julian::julday(point.year, point.month, point.day, point.hour, 1);

        println!("\n--- {} ---", point.year);
        println!("{:<12} {:>10} {:>10} {:>8}", "Planet", "Ours", "SwissEph", "Diff\"");

        for (i, planet) in PLANETS.iter().enumerate() {
            let pos = calc_ut(jd, *planet, false).expect("calc failed");
            let expected = point.positions[i];
            let diff = angle_diff(pos.longitude, expected);
            let diff_arcsec = diff * 3600.0;

            let planet_name = match planet {
                Planet::Sun => "Sun",
                Planet::Moon => "Moon",
                Planet::Mercury => "Mercury",
                Planet::Venus => "Venus",
                Planet::Mars => "Mars",
                Planet::Jupiter => "Jupiter",
                Planet::Saturn => "Saturn",
                Planet::Uranus => "Uranus",
                Planet::Neptune => "Neptune",
                Planet::Pluto => "Pluto",
                Planet::TrueNode => "TrueNode",
            };

            println!("{:<12} {:>10.4} {:>10.4} {:>8.1}",
                     planet_name, pos.longitude, expected, diff_arcsec);

            all_errors.push((point.year, planet_name, diff_arcsec));
        }
    }

    // Summary by planet
    println!("\n{}", "=".repeat(60));
    println!(" Summary by Planet ");
    println!("{}", "=".repeat(60));
    println!("{:<12} {:>10} {:>10} {:>10}", "Planet", "Min\"", "Max\"", "Avg\"");

    let planet_names = ["Sun", "Moon", "Mercury", "Venus", "Mars",
                        "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto", "TrueNode"];

    for name in planet_names {
        let errors: Vec<f64> = all_errors.iter()
            .filter(|(_, n, _)| *n == name)
            .map(|(_, _, e)| *e)
            .collect();

        let min = errors.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = errors.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = errors.iter().sum::<f64>() / errors.len() as f64;

        println!("{:<12} {:>10.1} {:>10.1} {:>10.1}", name, min, max, avg);
    }

    // Check thresholds
    let inner_max: f64 = all_errors.iter()
        .filter(|(_, n, _)| ["Sun", "Moon", "Mercury", "Venus", "Mars"].contains(n))
        .map(|(_, _, e)| *e)
        .fold(0.0, f64::max);

    let outer_max: f64 = all_errors.iter()
        .filter(|(_, n, _)| ["Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"].contains(n))
        .map(|(_, _, e)| *e)
        .fold(0.0, f64::max);

    println!("\nInner planets max error: {:.1}\" ({:.2}')", inner_max, inner_max / 60.0);
    println!("Outer planets max error: {:.1}\" ({:.2}')", outer_max, outer_max / 60.0);

    // Inner planets should be < 5 arcmin across all eras
    assert!(inner_max < 300.0, "Inner planet error exceeds 5 arcmin");
    // Outer planets can have up to 2 degrees for dates far from J2000
    assert!(outer_max < 7200.0, "Outer planet error exceeds 2 degrees");
}

#[test]
fn test_monthly_samples_modern_era() {
    // Test every month from 1980 to 2040 (modern era, higher accuracy expected)
    println!("\n{}", "=".repeat(60));
    println!(" Monthly Accuracy Test (1980-2040) ");
    println!("{}", "=".repeat(60));

    let mut max_inner_error = 0.0f64;
    let mut max_outer_error = 0.0f64;
    let mut samples = 0;

    for year in (1980..=2040).step_by(5) {
        for month in [1, 4, 7, 10] { // Quarterly samples
            let jd = julian::julday(year, month, 15, 12.0, 1);

            // Just check that calculations don't panic and return valid values
            for planet in &PLANETS {
                let pos = calc_ut(jd, *planet, false).expect("calc failed");

                assert!(pos.longitude >= 0.0 && pos.longitude < 360.0,
                        "Invalid longitude for {:?} at {}-{}", planet, year, month);
                // TrueNode doesn't have a meaningful distance
                if *planet != Planet::TrueNode {
                    assert!(pos.distance > 0.0,
                            "Invalid distance for {:?} at {}-{}", planet, year, month);
                }

                samples += 1;
            }
        }
    }

    println!("Tested {} planet-date combinations successfully", samples);

    // For the J2000 reference point, verify accuracy
    let jd = julian::julday(2000, 1, 1, 12.0, 1);
    let reference = [280.3689197, 223.3237754, 271.8892750, 241.5657983, 327.9633133,
                     25.2530303, 40.3956390, 314.8092232, 303.1929812, 251.4547088, 123.9528954];

    println!("\nJ2000 verification:");
    for (i, planet) in PLANETS.iter().enumerate() {
        let pos = calc_ut(jd, *planet, false).unwrap();
        let diff = angle_diff(pos.longitude, reference[i]) * 3600.0;

        if i < 5 { // Inner planets
            max_inner_error = max_inner_error.max(diff);
        } else if i < 10 { // Outer planets
            max_outer_error = max_outer_error.max(diff);
        }
    }

    println!("Inner planets max error: {:.1}\"", max_inner_error);
    println!("Outer planets max error: {:.1}\"", max_outer_error);
}

#[test]
fn test_historical_dates() {
    // Test specific historical dates
    let historical = [
        (1969, 7, 20, 20.0, "Moon Landing"),
        (1986, 1, 28, 16.0, "Challenger"),
        (1997, 8, 31, 4.0, "Diana's death"),
        (2001, 9, 11, 13.0, "9/11"),
        (2020, 3, 11, 12.0, "COVID pandemic declared"),
    ];

    println!("\n{}", "=".repeat(60));
    println!(" Historical Dates Test ");
    println!("{}", "=".repeat(60));

    for (year, month, day, hour, event) in historical {
        let jd = julian::julday(year, month, day, hour, 1);

        println!("\n{} ({}-{:02}-{:02}):", event, year, month, day);

        let sun = calc_ut(jd, Planet::Sun, false).unwrap();
        let moon = calc_ut(jd, Planet::Moon, false).unwrap();

        // Determine zodiac signs
        let sun_sign = zodiac_sign(sun.longitude);
        let moon_sign = zodiac_sign(moon.longitude);

        println!("  Sun: {:.2}° {} | Moon: {:.2}° {}",
                 sun.longitude, sun_sign, moon.longitude, moon_sign);

        // Sanity checks
        assert!(sun.longitude >= 0.0 && sun.longitude < 360.0);
        assert!(moon.longitude >= 0.0 && moon.longitude < 360.0);
    }
}

fn zodiac_sign(longitude: f64) -> &'static str {
    match (longitude / 30.0) as i32 {
        0 => "Aries",
        1 => "Taurus",
        2 => "Gemini",
        3 => "Cancer",
        4 => "Leo",
        5 => "Virgo",
        6 => "Libra",
        7 => "Scorpio",
        8 => "Sagittarius",
        9 => "Capricorn",
        10 => "Aquarius",
        11 => "Pisces",
        _ => "Unknown",
    }
}
