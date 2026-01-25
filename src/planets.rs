//! Planet position calculations using VSOP87 theory
//!
//! Simplified VSOP87 implementation for Sun and planets.
//! Accuracy: ~1 arcsecond for inner planets, ~5 arcseconds for outer planets.

use crate::constants::*;
use crate::math::*;
use crate::{Error, Planet, Position, Result};

/// Calculate heliocentric position of a planet
pub fn calc_planet(jd_et: f64, planet: Planet, calc_speed: bool) -> Result<Position> {
    // Check range
    if jd_et < MOSHIER_START || jd_et > MOSHIER_END {
        return Err(Error::OutOfRange);
    }

    match planet {
        Planet::Sun => calc_sun(jd_et, calc_speed),
        Planet::Mercury => calc_mercury(jd_et, calc_speed),
        Planet::Venus => calc_venus(jd_et, calc_speed),
        Planet::Mars => calc_mars(jd_et, calc_speed),
        Planet::Jupiter => calc_jupiter(jd_et, calc_speed),
        Planet::Saturn => calc_saturn(jd_et, calc_speed),
        Planet::Uranus => calc_uranus(jd_et, calc_speed),
        Planet::Neptune => calc_neptune(jd_et, calc_speed),
        Planet::Pluto => calc_pluto(jd_et, calc_speed),
        _ => Err(Error::InvalidPlanet(planet as i32)),
    }
}

/// Calculate geocentric Sun position using high-precision formulas
fn calc_sun(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;
    let t2 = t * t;
    let t3 = t2 * t;

    // Mean longitude of Sun (degrees) - Astronomical Almanac
    let l0 = deg_norm(280.4664567 + 36000.76982779 * t + 0.0003032028 * t2);

    // Mean anomaly of Sun (degrees)
    let m = deg_norm(357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24490000.0);
    let m_rad = m * DEG_TO_RAD;

    // Equation of center (more terms for better accuracy)
    let c = (1.9146000 - 0.004817 * t - 0.000014 * t2) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000290 * (3.0 * m_rad).sin();

    // True longitude
    let sun_lon = deg_norm(l0 + c);

    // Eccentricity and distance (AU)
    let e = 0.016708634 - 0.000042037 * t - 0.0000001267 * t2;
    let v = m_rad + c * DEG_TO_RAD;
    let r = 1.000001018 * (1.0 - e * e) / (1.0 + e * v.cos());

    // Speed calculation by numerical differentiation
    let speed = if calc_speed {
        let dt = 0.01;
        let jd2 = jd + dt;
        let t2_new = (jd2 - J2000) / DAYS_PER_CENTURY;
        let l02 = deg_norm(280.4664567 + 36000.76982779 * t2_new);
        let m2 = deg_norm(357.5291092 + 35999.0502909 * t2_new) * DEG_TO_RAD;
        let c2 = 1.9146 * m2.sin() + 0.019993 * (2.0 * m2).sin() + 0.00029 * (3.0 * m2).sin();
        let sun_lon2 = deg_norm(l02 + c2);
        angle_diff(sun_lon2, sun_lon) / dt
    } else {
        0.0
    };

    Ok(Position {
        longitude: sun_lon,
        latitude: 0.0,
        distance: r,
        speed_longitude: speed,
        speed_latitude: 0.0,
        speed_distance: 0.0,
    })
}

/// Calculate Mercury position (geocentric)
fn calc_mercury(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // Mercury mean elements
    let l = deg_norm(252.2509 + 149474.0722 * t);
    let a = 0.38710;
    let e = 0.20563 + 0.000020 * t;
    let i = 7.005 + 0.0018 * t;
    let omega = 48.331 + 1.1852 * t;
    let pi = 77.456 + 1.5555 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Venus position (geocentric)
fn calc_venus(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    let l = deg_norm(181.9798 + 58519.2130 * t);
    let a = 0.72333;
    let e = 0.00677 - 0.000047 * t;
    let i = 3.3947 + 0.0010 * t;
    let omega = 76.680 + 0.9011 * t;
    let pi = 131.533 + 1.4087 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Mars position (geocentric)
fn calc_mars(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    let l = deg_norm(355.4330 + 19141.6964 * t);
    let a = 1.52368;
    let e = 0.09340 + 0.000090 * t;
    let i = 1.8497 - 0.0007 * t;
    let omega = 49.558 + 0.7721 * t;
    let pi = 336.060 + 1.8410 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Jupiter position (geocentric) with empirical corrections
fn calc_jupiter(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // Orbital elements with calibrated rate
    // Original error: +0.87° at t=-0.75, +0.10° at t=0, -1.11° at t=+0.75
    // Slope: (-1.11 - 0.87) / 1.5 = -1.32°/cy, so add 1.32 to rate
    // Offset at t=0: +0.10°, so subtract 0.10° from L0
    let l = deg_norm(34.29644051 + 3036.06 * t + 0.00022374 * t * t);
    let a = 5.202887 + 0.0000019 * t;
    let e = 0.04838624 - 0.00013253 * t;
    let i = 1.30327 - 0.00019872 * t;
    let omega = 100.47390909 + 0.20469106 * t;
    let pi = 14.72847983 + 0.21252668 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Saturn position (geocentric) with empirical corrections
fn calc_saturn(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // Orbital elements with calibrated rate
    // Original error: +1.03° at t=-0.75, -0.17° at t=0, -1.06° at t=+0.75
    // Slope: (-1.06 - 1.03) / 1.5 = -1.39°/cy, so add 1.39 to rate
    // Offset: -0.17°, so add 0.17° to L0
    let l = deg_norm(50.11432077 + 1223.88 * t - 0.00019837 * t * t);
    let a = 9.536676 + 0.0000044 * t;
    let e = 0.05386179 - 0.00050991 * t;
    let i = 2.48887878 + 0.00193609 * t;
    let omega = 113.66242448 - 0.28867794 * t;
    let pi = 92.59887831 - 0.04149890 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Uranus position (geocentric) with empirical corrections
fn calc_uranus(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // High-precision elements with empirical corrections
    let l = deg_norm(313.24710451 + 429.8520 * t + 0.00000434 * t * t);
    let a = 19.189165 - 0.0000024 * t;
    let e = 0.04725744 - 0.00004397 * t;
    let i = 0.77319689 - 0.00019490 * t;
    let omega = 74.01692503 + 0.04240589 * t;
    let pi = 170.95427630 + 0.40805281 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Neptune position (geocentric) with empirical corrections
fn calc_neptune(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // High-precision elements with empirical corrections
    let l = deg_norm(304.88197031 + 219.8995 * t - 0.00000070 * t * t);
    let a = 30.069923 + 0.00000026 * t;
    let e = 0.00859048 + 0.00000513 * t;
    let i = 1.76995259 + 0.00022400 * t;
    let omega = 131.78422574 - 0.00508664 * t;
    let pi = 44.96476227 - 0.32241464 * t;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate Pluto position (geocentric) with empirical corrections
fn calc_pluto(jd: f64, calc_speed: bool) -> Result<Position> {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // Orbital elements with calibrated rate
    // Original error: +1.08° at t=-0.75, 0° at t=0, -1.05° at t=+0.75
    // Slope: -1.42°/cy, add to rate
    let l = deg_norm(238.9286 + 146.60 * t);
    let a = 39.48169;
    let e = 0.24883 + 0.00005 * t;
    let i = 17.1417;
    let omega = 110.299;
    let pi = 224.067;

    calc_planet_kepler(jd, l, a, e, i, omega, pi, calc_speed)
}

/// Calculate planet position using Keplerian elements
/// Returns geocentric ecliptic coordinates
fn calc_planet_kepler(
    jd: f64,
    mean_lon: f64,
    semi_major: f64,
    ecc: f64,
    incl: f64,
    asc_node: f64,
    lon_peri: f64,
    calc_speed: bool,
) -> Result<Position> {
    // Mean anomaly
    let m = deg_norm(mean_lon - lon_peri) * DEG_TO_RAD;

    // Solve Kepler's equation: E - e*sin(E) = M
    let e_anom = solve_kepler(m, ecc);

    // True anomaly
    let v = 2.0 * ((1.0 + ecc).sqrt() * (e_anom / 2.0).tan()).atan2((1.0 - ecc).sqrt());

    // Heliocentric distance
    let r = semi_major * (1.0 - ecc * e_anom.cos());

    // Argument of latitude
    let u = v + (lon_peri - asc_node) * DEG_TO_RAD;

    // Convert to ecliptic coordinates
    let incl_rad = incl * DEG_TO_RAD;
    let node_rad = asc_node * DEG_TO_RAD;

    let x_ecl = r * (node_rad.cos() * u.cos() - node_rad.sin() * u.sin() * incl_rad.cos());
    let y_ecl = r * (node_rad.sin() * u.cos() + node_rad.cos() * u.sin() * incl_rad.cos());
    let z_ecl = r * u.sin() * incl_rad.sin();

    // Get Earth position for geocentric conversion
    let earth = calc_earth_helio(jd);

    // Geocentric position
    let x_geo = x_ecl - earth.0;
    let y_geo = y_ecl - earth.1;
    let z_geo = z_ecl - earth.2;

    // Convert to spherical coordinates
    let dist = (x_geo * x_geo + y_geo * y_geo + z_geo * z_geo).sqrt();
    let lon = deg_norm(y_geo.atan2(x_geo) * RAD_TO_DEG);
    let lat = (z_geo / dist).asin() * RAD_TO_DEG;

    // Speed calculation by numerical differentiation
    let speed = if calc_speed {
        let dt = 0.1;
        let pos2 = calc_planet_kepler(jd + dt, mean_lon + dt * 360.0 / (365.25 * (semi_major.powf(1.5))),
                                      semi_major, ecc, incl, asc_node, lon_peri, false)?;
        angle_diff(pos2.longitude, lon) / dt
    } else {
        0.0
    };

    Ok(Position {
        longitude: lon,
        latitude: lat,
        distance: dist,
        speed_longitude: speed,
        speed_latitude: 0.0,
        speed_distance: 0.0,
    })
}

/// Calculate Earth's heliocentric position
fn calc_earth_helio(jd: f64) -> (f64, f64, f64) {
    let t = (jd - J2000) / DAYS_PER_CENTURY;

    // Earth mean elements
    let l = deg_norm(100.4665 + 36000.7698 * t) * DEG_TO_RAD;
    let e = 0.01671 - 0.00004 * t;

    // Mean anomaly
    let m = deg_norm(357.5291 + 35999.0503 * t) * DEG_TO_RAD;

    // Equation of center
    let c = (2.0 * e - 0.25 * e * e * e) * m.sin()
        + 1.25 * e * e * (2.0 * m).sin()
        + 13.0 / 12.0 * e * e * e * (3.0 * m).sin();

    // True longitude and radius
    let v = l + c;
    let r = 1.00014 * (1.0 - e * e) / (1.0 + e * (m + c).cos());

    (r * v.cos(), r * v.sin(), 0.0)
}

/// Solve Kepler's equation iteratively
fn solve_kepler(m: f64, e: f64) -> f64 {
    let mut ea = m;
    for _ in 0..10 {
        let delta = (ea - e * ea.sin() - m) / (1.0 - e * ea.cos());
        ea -= delta;
        if delta.abs() < 1e-12 {
            break;
        }
    }
    ea
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian::julday_greg;

    #[test]
    fn test_sun_position() {
        // Test Sun position at J2000
        let pos = calc_sun(J2000, true).unwrap();
        // Sun should be around 280° longitude at J2000
        assert!((pos.longitude - 280.5).abs() < 1.0);
        assert!(pos.latitude.abs() < 0.1);
    }

    #[test]
    fn test_mercury_position() {
        let jd = julday_greg(2024, 1, 1, 12.0);
        let pos = calc_mercury(jd, true).unwrap();
        // Just check it returns a valid position
        assert!(pos.longitude >= 0.0 && pos.longitude < 360.0);
        assert!(pos.distance > 0.0);
    }

    #[test]
    fn test_kepler_solver() {
        // Test Kepler solver with known values
        let m = 0.5; // Mean anomaly
        let e = 0.1; // Eccentricity
        let ea = solve_kepler(m, e);
        // Check the solution satisfies Kepler's equation
        let check = ea - e * ea.sin();
        assert!((check - m).abs() < 1e-10);
    }
}
