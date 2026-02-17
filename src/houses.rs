//! House cusp calculations
//!
//! Implements Placidus house system.

use crate::constants::*;
use crate::math::*;
use crate::{delta_t, Houses, Result};

/// Calculate Placidus house cusps
pub fn calc_houses_placidus(jd_ut: f64, lat: f64, lon: f64) -> Result<Houses> {
    // Convert to ephemeris time
    let jd_et = jd_ut + delta_t(jd_ut);

    // Obliquity of ecliptic
    let eps = obliquity(jd_et);

    // ARMC (local sidereal time in degrees)
    let armc_deg = armc(jd_ut, lon);
    let armc_rad = armc_deg * DEG_TO_RAD;

    // Latitude in radians
    let lat_rad = lat * DEG_TO_RAD;

    // Calculate MC (Midheaven)
    let mc = calc_mc(armc_rad, eps);

    // Calculate Ascendant
    let asc = calc_ascendant(armc_rad, lat_rad, eps);

    // Calculate intermediate cusps using Placidus method
    let cusps = calc_placidus_cusps(armc_deg, lat_rad, eps, mc, asc);

    // Calculate Vertex
    let vertex = calc_vertex(armc_rad, lat_rad, eps);

    Ok(Houses {
        cusps,
        ascendant: deg_norm(asc * RAD_TO_DEG),
        mc: deg_norm(mc * RAD_TO_DEG),
        armc: armc_deg,
        vertex: deg_norm(vertex * RAD_TO_DEG),
    })
}

/// Calculate MC (Medium Coeli / Midheaven)
fn calc_mc(armc: f64, eps: f64) -> f64 {
    let (sin_armc, cos_armc) = armc.sin_cos();
    let mut mc = sin_armc.atan2(cos_armc * eps.cos());

    // atan2 already resolves the correct quadrant for MC because
    // cos(ε) is always positive (~0.917), so the sign of
    // cos(ARMC)*cos(ε) matches cos(ARMC). We only need to
    // normalize negative results to [0, 2π).
    if mc < 0.0 {
        mc += TWOPI;
    }

    mc
}

/// Calculate Ascendant
fn calc_ascendant(armc: f64, lat: f64, eps: f64) -> f64 {
    let (sin_eps, cos_eps) = eps.sin_cos();
    let (sin_armc, cos_armc) = armc.sin_cos();
    let tan_lat = lat.tan();

    let y = -cos_armc;
    let x = sin_armc * cos_eps + tan_lat * sin_eps;

    let mut asc = y.atan2(x);

    // The ascendant must be in the half of the sky that is rising.
    // atan2 returns the correct angle modulo π, but can be off by 180°.
    // Adding π selects the rising point (ascendant) instead of the
    // setting point (descendant).
    asc += std::f64::consts::PI;
    if asc < 0.0 {
        asc += TWOPI;
    }
    if asc >= TWOPI {
        asc -= TWOPI;
    }

    asc
}

/// Calculate Vertex (western horizon intersection with prime vertical)
fn calc_vertex(armc: f64, lat: f64, eps: f64) -> f64 {
    // Vertex is the Ascendant for the co-latitude
    let co_lat = std::f64::consts::FRAC_PI_2 - lat;
    let armc_vtx = armc + std::f64::consts::PI;
    calc_ascendant(armc_vtx, co_lat, eps)
}

/// Calculate Placidus house cusps
///
/// Uses the Swiss Ephemeris algorithm: for each intermediate cusp, iterate
/// on the Placidus proportional semi-arc condition using pole-height
/// refinement via `asc1_deg`.
fn calc_placidus_cusps(armc_deg: f64, lat: f64, eps: f64, mc: f64, asc: f64) -> [f64; 13] {
    let mut cusps = [0.0; 13];

    // Cusp 1 = Ascendant
    cusps[1] = deg_norm(asc * RAD_TO_DEG);

    // Cusp 10 = MC
    cusps[10] = deg_norm(mc * RAD_TO_DEG);

    // Cusp 4 = IC (opposite MC)
    cusps[4] = deg_norm((mc + std::f64::consts::PI) * RAD_TO_DEG);

    // Cusp 7 = Descendant (opposite Ascendant)
    cusps[7] = deg_norm((asc + std::f64::consts::PI) * RAD_TO_DEG);

    let (sin_eps, cos_eps) = eps.sin_cos();
    let tan_lat = lat.tan();
    let tan_eps = eps.tan();

    // Initial pole height estimates (seeds for the iteration).
    // a = ascensional difference at the obliquity circle
    let a_arg = (tan_lat * tan_eps).clamp(-1.0, 1.0);
    let a = a_arg.asin(); // radians
    let fh1 = if tan_eps.abs() > 1e-15 {
        ((a / 3.0).sin() / tan_eps).atan() * RAD_TO_DEG
    } else {
        0.0
    };
    let fh2 = if tan_eps.abs() > 1e-15 {
        ((a * 2.0 / 3.0).sin() / tan_eps).atan() * RAD_TO_DEG
    } else {
        0.0
    };

    // Cusp parameters: (cusp_number, RA offset from ARMC, divisor, initial pole)
    // Offsets: 30° and 60° for upper cusps (11, 12), 120° and 150° for lower (2, 3)
    // Divisors: 3.0 encodes 1/3 semi-arc fraction, 1.5 encodes 2/3
    let cusp_params: [(usize, f64, f64, f64); 4] = [
        (11, 30.0, 3.0, fh1),
        (12, 60.0, 1.5, fh2),
        (2, 120.0, 1.5, fh2),
        (3, 150.0, 3.0, fh1),
    ];

    for (cusp_num, offset, divisor, initial_f) in cusp_params {
        let rectasc = deg_norm(armc_deg + offset);
        cusps[cusp_num] =
            placidus_cusp_deg(rectasc, tan_lat, sin_eps, cos_eps, divisor, initial_f);
    }

    // Opposite cusps
    cusps[5] = deg_norm(cusps[11] + 180.0);
    cusps[6] = deg_norm(cusps[12] + 180.0);
    cusps[8] = deg_norm(cusps[2] + 180.0);
    cusps[9] = deg_norm(cusps[3] + 180.0);

    cusps
}

/// Compute a single Placidus intermediate cusp (in degrees).
///
/// Implements the Swiss Ephemeris pole-height iteration: at each step,
/// compute the declination of the current cusp estimate, derive the
/// ascensional difference, apply the Placidus proportional condition
/// (dividing by `divisor`), compute a new pole height, and convert
/// back to ecliptic longitude via `asc1_deg`.
fn placidus_cusp_deg(
    rectasc: f64,
    tan_lat: f64,
    sin_eps: f64,
    cos_eps: f64,
    divisor: f64,
    initial_f: f64,
) -> f64 {
    const VERY_SMALL: f64 = 1e-15;
    const CONVERGENCE: f64 = 1.0 / 360000.0; // ~0.01 arcsecond
    const MAX_ITER: usize = 100;

    // Seed value from initial pole height estimate
    let mut cusp = asc1_deg(rectasc, initial_f, sin_eps, cos_eps);

    let mut prev_cusp = 0.0_f64;
    for i in 0..MAX_ITER {
        // Declination of the current cusp estimate
        let sin_cusp = (cusp * DEG_TO_RAD).sin();
        let decl = (sin_eps * sin_cusp).asin();
        let tan_decl = decl.tan();

        if tan_decl.abs() < VERY_SMALL {
            // Cusp is on the equator — degenerate case
            cusp = rectasc;
            break;
        }

        // Ascensional difference: AD = asin(tan(lat) * tan(decl))
        let ad_arg = tan_lat * tan_decl;
        if ad_arg.abs() > 1.0 {
            // Circumpolar — Placidus fails, fall back to equal cusp
            cusp = rectasc;
            break;
        }
        let ad = ad_arg.asin(); // radians

        // New pole height from the Placidus proportional condition:
        // f = atan(sin(AD / divisor) / tan(decl))
        let f_deg = ((ad / divisor).sin() / tan_decl).atan() * RAD_TO_DEG;

        // New cusp estimate
        cusp = asc1_deg(rectasc, f_deg, sin_eps, cos_eps);

        // Convergence check (skip first iteration)
        if i > 0 {
            let diff = (cusp - prev_cusp).abs();
            let diff = if diff > 180.0 { 360.0 - diff } else { diff };
            if diff < CONVERGENCE {
                break;
            }
        }
        prev_cusp = cusp;
    }

    cusp
}

/// Convert right ascension to ecliptic longitude for RA in [0°, 90°].
///
/// Computes atan(sin(x) / (-tan(f)*sin(eps) + cos(eps)*cos(x)))
/// with explicit quadrant handling. Returns result in [0°, 180°).
/// Based on Swiss Ephemeris `Asc2`.
fn asc2_deg(x: f64, f: f64, sin_eps: f64, cos_eps: f64) -> f64 {
    const VERY_SMALL: f64 = 1e-15;

    let x_rad = x * DEG_TO_RAD;
    let f_rad = f * DEG_TO_RAD;

    let mut denom = -f_rad.tan() * sin_eps + cos_eps * x_rad.cos();
    let mut numer = x_rad.sin();

    if denom.abs() < VERY_SMALL {
        denom = 0.0;
    }
    if numer.abs() < VERY_SMALL {
        numer = 0.0;
    }

    let mut ass = if numer == 0.0 {
        if denom < 0.0 {
            -VERY_SMALL
        } else {
            VERY_SMALL
        }
    } else if denom == 0.0 {
        if numer < 0.0 {
            -90.0
        } else {
            90.0
        }
    } else {
        (numer / denom).atan() * RAD_TO_DEG
    };

    // When denominator is negative, atan gives (-90°, 0°); add 180° to map to (90°, 180°)
    if ass < 0.0 {
        ass += 180.0;
    }

    ass
}

/// Convert right ascension to ecliptic longitude for the full 0°..360° range.
///
/// Extends `asc2_deg` to all four quadrants by reflecting into Q1 and back.
/// Based on Swiss Ephemeris `Asc1`.
fn asc1_deg(x1: f64, f: f64, sin_eps: f64, cos_eps: f64) -> f64 {
    const VERY_SMALL: f64 = 1e-15;

    let x1 = deg_norm(x1);
    let n = (x1 / 90.0) as i32 + 1;

    if (90.0 - f).abs() < VERY_SMALL {
        return 180.0;
    }
    if (90.0 + f).abs() < VERY_SMALL {
        return 0.0;
    }

    let ass = match n {
        1 => asc2_deg(x1, f, sin_eps, cos_eps),
        2 => 180.0 - asc2_deg(180.0 - x1, -f, sin_eps, cos_eps),
        3 => 180.0 + asc2_deg(x1 - 180.0, -f, sin_eps, cos_eps),
        _ => 360.0 - asc2_deg(360.0 - x1, f, sin_eps, cos_eps),
    };

    let mut ass = deg_norm(ass);

    // Snap to exact quadrant boundaries to avoid floating-point drift
    if (ass - 90.0).abs() < VERY_SMALL {
        ass = 90.0;
    }
    if (ass - 180.0).abs() < VERY_SMALL {
        ass = 180.0;
    }
    if (ass - 270.0).abs() < VERY_SMALL {
        ass = 270.0;
    }
    if (ass - 360.0).abs() < VERY_SMALL {
        ass = 0.0;
    }

    ass
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian::julday_greg;

    #[test]
    fn test_houses_basic() {
        // Test at a known location and time
        let jd = julday_greg(2000, 1, 1, 12.0);
        let houses = calc_houses_placidus(jd, 47.38, 8.54).unwrap();

        // Basic sanity checks
        assert!(houses.ascendant >= 0.0 && houses.ascendant < 360.0);
        assert!(houses.mc >= 0.0 && houses.mc < 360.0);

        // ASC and DESC should be opposite
        let desc = deg_norm(houses.ascendant + 180.0);
        assert!((houses.cusps[7] - desc).abs() < 0.01);

        // MC and IC should be opposite
        let ic = deg_norm(houses.mc + 180.0);
        assert!((houses.cusps[4] - ic).abs() < 0.01);
    }

    #[test]
    fn test_ascendant_against_reference() {
        // London, 2000-01-01 12:00 UT
        // Swiss Ephemeris reference: ASC = 24.01°, MC = 280.47°
        let jd = julday_greg(2000, 1, 1, 12.0);
        let houses = calc_houses_placidus(jd, 51.5074, -0.1278).unwrap();

        // ASC should be ~24° (Aries), not ~204° (Libra)
        assert!((houses.ascendant - 24.01).abs() < 0.5,
            "ASC was {}°, expected ~24.01°", houses.ascendant);

        // MC should be ~280° (Capricorn), not ~100° (Cancer)
        // Tolerance of 1.0° accounts for VSOP87 vs Swiss Ephemeris differences
        // in obliquity and delta-T models
        assert!((houses.mc - 280.47).abs() < 1.0,
            "MC was {}°, expected ~280.47°", houses.mc);
    }

    #[test]
    fn test_mc_calculation() {
        // At ARMC = 0, MC should be 0° (Aries point)
        let eps = OBLIQUITY_J2000 * DEG_TO_RAD;
        let mc = calc_mc(0.0, eps);
        assert!(mc.abs() < 0.01);
    }

    #[test]
    fn test_ascendant_calculation() {
        // Basic ascendant test
        let eps = OBLIQUITY_J2000 * DEG_TO_RAD;
        let lat = 45.0 * DEG_TO_RAD;
        let asc = calc_ascendant(0.0, lat, eps);

        // ASC should be in a reasonable range
        assert!(asc >= 0.0 && asc < TWOPI);
    }
}
