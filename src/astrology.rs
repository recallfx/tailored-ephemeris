//! Astrological calculations derived from ephemeris data
//!
//! This module provides higher-level astrological calculations:
//! - Zodiac sign from longitude
//! - Moon phases
//! - Aspects between planets
//! - Planetary hours
//! - Void-of-course Moon detection

use crate::{calc_ut, calc_heliocentric_ut, calc_houses, Planet, Result};

/// Zodiac signs in order (0 = Aries, 11 = Pisces)
pub const ZODIAC_SIGNS: [&str; 12] = [
    "aries", "taurus", "gemini", "cancer", "leo", "virgo",
    "libra", "scorpio", "sagittarius", "capricorn", "aquarius", "pisces",
];

/// Planet keys matching the order used in the API
pub const PLANET_KEYS: [&str; 10] = [
    "sun", "moon", "mercury", "venus", "mars",
    "jupiter", "saturn", "uranus", "neptune", "pluto",
];

/// Moon phase keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl MoonPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            MoonPhase::NewMoon => "new_moon",
            MoonPhase::WaxingCrescent => "waxing_crescent",
            MoonPhase::FirstQuarter => "first_quarter",
            MoonPhase::WaxingGibbous => "waxing_gibbous",
            MoonPhase::FullMoon => "full_moon",
            MoonPhase::WaningGibbous => "waning_gibbous",
            MoonPhase::LastQuarter => "last_quarter",
            MoonPhase::WaningCrescent => "waning_crescent",
        }
    }
}

/// Aspect types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectType {
    Conjunction,
    Sextile,
    Square,
    Trine,
    Opposition,
    Quincunx,
    SemiSextile,
    SemiSquare,
    Sesquiquadrate,
    Quintile,
}

impl AspectType {
    pub fn angle(&self) -> f64 {
        match self {
            AspectType::Conjunction => 0.0,
            AspectType::SemiSextile => 30.0,
            AspectType::SemiSquare => 45.0,
            AspectType::Sextile => 60.0,
            AspectType::Quintile => 72.0,
            AspectType::Square => 90.0,
            AspectType::Trine => 120.0,
            AspectType::Sesquiquadrate => 135.0,
            AspectType::Quincunx => 150.0,
            AspectType::Opposition => 180.0,
        }
    }

    pub fn default_orb(&self) -> f64 {
        match self {
            AspectType::Conjunction => 8.0,
            AspectType::Opposition => 8.0,
            AspectType::Square => 8.0,
            AspectType::Trine => 8.0,
            AspectType::Sextile => 6.0,
            AspectType::Quincunx => 5.0,
            AspectType::SemiSextile => 4.0,
            AspectType::SemiSquare => 4.0,
            AspectType::Sesquiquadrate => 4.0,
            AspectType::Quintile => 4.0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AspectType::Conjunction => "conjunction",
            AspectType::Sextile => "sextile",
            AspectType::Square => "square",
            AspectType::Trine => "trine",
            AspectType::Opposition => "opposition",
            AspectType::Quincunx => "quincunx",
            AspectType::SemiSextile => "semi-sextile",
            AspectType::SemiSquare => "semi-square",
            AspectType::Sesquiquadrate => "sesquiquadrate",
            AspectType::Quintile => "quintile",
        }
    }

    pub fn all() -> &'static [AspectType] {
        &[
            AspectType::Conjunction,
            AspectType::Sextile,
            AspectType::Square,
            AspectType::Trine,
            AspectType::Opposition,
            AspectType::Quincunx,
            AspectType::SemiSextile,
            AspectType::SemiSquare,
            AspectType::Sesquiquadrate,
            AspectType::Quintile,
        ]
    }
}

/// Computed aspect between two planets
#[derive(Debug, Clone)]
pub struct ComputedAspect {
    pub planet1_key: &'static str,
    pub planet2_key: &'static str,
    pub aspect_type: AspectType,
    pub orb: f64,
    pub is_applying: bool,
}

/// Configuration for aspect orbs
/// All values are in degrees
#[derive(Debug, Clone)]
pub struct OrbConfig {
    pub conjunction: f64,
    pub opposition: f64,
    pub square: f64,
    pub trine: f64,
    pub sextile: f64,
    pub quincunx: f64,
    pub semi_sextile: f64,
    pub semi_square: f64,
    pub sesquiquadrate: f64,
    pub quintile: f64,
}

impl Default for OrbConfig {
    fn default() -> Self {
        OrbConfig {
            conjunction: 8.0,
            opposition: 8.0,
            square: 8.0,
            trine: 8.0,
            sextile: 6.0,
            quincunx: 5.0,
            semi_sextile: 4.0,
            semi_square: 4.0,
            sesquiquadrate: 4.0,
            quintile: 4.0,
        }
    }
}

impl OrbConfig {
    /// Get orb for a specific aspect type
    pub fn get_orb(&self, aspect: AspectType) -> f64 {
        match aspect {
            AspectType::Conjunction => self.conjunction,
            AspectType::Opposition => self.opposition,
            AspectType::Square => self.square,
            AspectType::Trine => self.trine,
            AspectType::Sextile => self.sextile,
            AspectType::Quincunx => self.quincunx,
            AspectType::SemiSextile => self.semi_sextile,
            AspectType::SemiSquare => self.semi_square,
            AspectType::Sesquiquadrate => self.sesquiquadrate,
            AspectType::Quintile => self.quintile,
        }
    }
}

/// Planet position with derived data
#[derive(Debug, Clone)]
pub struct PlanetPosition {
    pub planet_key: &'static str,
    pub longitude: f64,
    pub sign_key: &'static str,
    pub sign_degree: f64,
    pub is_retrograde: bool,
    pub speed: f64,
}

/// House cusp with derived data
#[derive(Debug, Clone)]
pub struct HouseCusp {
    pub house_number: u8,
    pub cusp_longitude: f64,
    pub sign_key: &'static str,
    pub sign_degree: f64,
}

/// Complete natal chart
#[derive(Debug, Clone)]
pub struct NatalChart {
    pub planets: Vec<PlanetPosition>,
    pub houses: Vec<HouseCusp>,
    pub ascendant: f64,
    pub midheaven: f64,
    pub north_node_longitude: f64,
    pub north_node_sign: &'static str,
    pub north_node_degree: f64,
}

/// Chaldean order for planetary hours
const CHALDEAN_ORDER: [&str; 7] = [
    "saturn", "jupiter", "mars", "sun", "venus", "mercury", "moon"
];

/// Day rulers (0 = Sunday)
const DAY_RULERS: [&str; 7] = [
    "sun", "moon", "mars", "mercury", "jupiter", "venus", "saturn"
];

// ============================================================================
// Core Functions
// ============================================================================

/// Get zodiac sign from ecliptic longitude
pub fn get_sign_from_longitude(longitude: f64) -> &'static str {
    let mut lon = longitude % 360.0;
    if lon < 0.0 { lon += 360.0; }
    let index = (lon / 30.0) as usize % 12;
    ZODIAC_SIGNS[index]
}

/// Get degree within sign (0-30)
pub fn get_sign_degree(longitude: f64) -> f64 {
    let mut lon = longitude % 360.0;
    if lon < 0.0 { lon += 360.0; }
    lon % 30.0
}

/// Calculate moon phase from Sun and Moon longitudes
pub fn get_moon_phase(sun_longitude: f64, moon_longitude: f64) -> MoonPhase {
    let mut diff = moon_longitude - sun_longitude;
    if diff < 0.0 { diff += 360.0; }
    if diff >= 360.0 { diff -= 360.0; }

    if diff < 45.0 { MoonPhase::NewMoon }
    else if diff < 90.0 { MoonPhase::WaxingCrescent }
    else if diff < 135.0 { MoonPhase::FirstQuarter }
    else if diff < 180.0 { MoonPhase::WaxingGibbous }
    else if diff < 225.0 { MoonPhase::FullMoon }
    else if diff < 270.0 { MoonPhase::WaningGibbous }
    else if diff < 315.0 { MoonPhase::LastQuarter }
    else { MoonPhase::WaningCrescent }
}

/// Calculate moon phase for a given date
pub fn calculate_moon_phase(jd: f64) -> Result<MoonPhase> {
    let sun = calc_ut(jd, Planet::Sun, false)?;
    let moon = calc_ut(jd, Planet::Moon, false)?;
    Ok(get_moon_phase(sun.longitude, moon.longitude))
}

/// Get planetary hour ruler for a given date/time
pub fn get_planetary_hour_ruler(year: i32, month: i32, day: i32, hour: u32) -> &'static str {
    // Calculate day of week (0 = Sunday)
    // Using Zeller's congruence (simplified for Gregorian)
    let mut y = year;
    let mut m = month;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let q = day;
    let k = y % 100;
    let j = y / 100;
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    let day_of_week = ((h + 6) % 7) as usize; // Convert to 0=Sunday

    let day_ruler = DAY_RULERS[day_of_week];
    let start_index = CHALDEAN_ORDER.iter().position(|&p| p == day_ruler).unwrap_or(0);
    let hour_index = (start_index + hour as usize) % 7;

    CHALDEAN_ORDER[hour_index]
}

/// Check if two angles form an aspect within orb
fn check_aspect(lon1: f64, lon2: f64, aspect: AspectType, orb: f64) -> Option<f64> {
    let mut diff = (lon1 - lon2).abs();
    if diff > 180.0 { diff = 360.0 - diff; }

    let actual_orb = (diff - aspect.angle()).abs();
    if actual_orb <= orb {
        Some(actual_orb)
    } else {
        None
    }
}

/// Compute aspects between two sets of positions with configurable orbs
pub fn compute_aspects_with_orbs(
    chart1: &[PlanetPosition],
    chart2: &[PlanetPosition],
    orb_config: &OrbConfig,
) -> Vec<ComputedAspect> {
    let mut aspects = Vec::new();
    let same_chart = std::ptr::eq(chart1.as_ptr(), chart2.as_ptr());

    for p1 in chart1 {
        for p2 in chart2 {
            // Skip same planet if same chart
            if same_chart && p1.planet_key == p2.planet_key {
                continue;
            }

            for &aspect_type in AspectType::all() {
                let orb = orb_config.get_orb(aspect_type);
                if let Some(actual_orb) = check_aspect(p1.longitude, p2.longitude, aspect_type, orb) {
                    // Determine if applying or separating
                    let relative_speed = if same_chart {
                        p1.speed - p2.speed
                    } else {
                        p1.speed
                    };

                    let mut diff = (p1.longitude - p2.longitude).abs();
                    if diff > 180.0 { diff = 360.0 - diff; }

                    let is_applying =
                        (diff > aspect_type.angle() && relative_speed < 0.0) ||
                        (diff < aspect_type.angle() && relative_speed > 0.0);

                    aspects.push(ComputedAspect {
                        planet1_key: p1.planet_key,
                        planet2_key: p2.planet_key,
                        aspect_type,
                        orb: actual_orb,
                        is_applying,
                    });
                }
            }
        }
    }

    aspects
}

/// Compute aspects between two sets of positions using default orbs
pub fn compute_aspects(
    chart1: &[PlanetPosition],
    chart2: &[PlanetPosition],
) -> Vec<ComputedAspect> {
    compute_aspects_with_orbs(chart1, chart2, &OrbConfig::default())
}

/// Get all planetary positions at a given time
pub fn get_all_planetary_positions(jd: f64) -> Result<Vec<PlanetPosition>> {
    let planets = [
        (Planet::Sun, "sun"),
        (Planet::Moon, "moon"),
        (Planet::Mercury, "mercury"),
        (Planet::Venus, "venus"),
        (Planet::Mars, "mars"),
        (Planet::Jupiter, "jupiter"),
        (Planet::Saturn, "saturn"),
        (Planet::Uranus, "uranus"),
        (Planet::Neptune, "neptune"),
        (Planet::Pluto, "pluto"),
    ];

    let mut positions = Vec::with_capacity(10);

    for (planet, key) in planets {
        let pos = calc_ut(jd, planet, true)?;
        positions.push(PlanetPosition {
            planet_key: key,
            longitude: pos.longitude,
            sign_key: get_sign_from_longitude(pos.longitude),
            sign_degree: get_sign_degree(pos.longitude),
            is_retrograde: pos.speed_longitude < 0.0,
            speed: pos.speed_longitude,
        });
    }

    Ok(positions)
}

/// Get complete natal chart
pub fn get_natal_chart(jd: f64, latitude: f64, longitude: f64) -> Result<NatalChart> {
    // Get planet positions
    let planets = get_all_planetary_positions(jd)?;

    // Get houses
    let house_data = calc_houses(jd, latitude, longitude)?;

    let mut houses = Vec::with_capacity(12);
    for i in 1..=12 {
        let cusp = house_data.cusps[i];
        houses.push(HouseCusp {
            house_number: i as u8,
            cusp_longitude: cusp,
            sign_key: get_sign_from_longitude(cusp),
            sign_degree: get_sign_degree(cusp),
        });
    }

    // Get North Node
    let node = calc_ut(jd, Planet::TrueNode, false)?;

    Ok(NatalChart {
        planets,
        houses,
        ascendant: house_data.ascendant,
        midheaven: house_data.mc,
        north_node_longitude: node.longitude,
        north_node_sign: get_sign_from_longitude(node.longitude),
        north_node_degree: get_sign_degree(node.longitude),
    })
}

/// Heliocentric chart (planets only, no houses/angles)
#[derive(Debug, Clone)]
pub struct HeliocentricChart {
    pub planets: Vec<PlanetPosition>,
}

/// Planet keys for heliocentric calculations (Earth + Mercury through Pluto)
pub const HELIOCENTRIC_PLANET_KEYS: [(&str, Planet); 9] = [
    ("earth", Planet::Earth),
    ("mercury", Planet::Mercury),
    ("venus", Planet::Venus),
    ("mars", Planet::Mars),
    ("jupiter", Planet::Jupiter),
    ("saturn", Planet::Saturn),
    ("uranus", Planet::Uranus),
    ("neptune", Planet::Neptune),
    ("pluto", Planet::Pluto),
];

/// Get all heliocentric planetary positions at a given time
pub fn get_all_heliocentric_positions(jd: f64) -> Result<Vec<PlanetPosition>> {
    let mut positions = Vec::with_capacity(9);

    for &(key, planet) in &HELIOCENTRIC_PLANET_KEYS {
        let pos = calc_heliocentric_ut(jd, planet, true)?;
        positions.push(PlanetPosition {
            planet_key: key,
            longitude: pos.longitude,
            sign_key: get_sign_from_longitude(pos.longitude),
            sign_degree: get_sign_degree(pos.longitude),
            is_retrograde: false, // No retrograde in heliocentric frame
            speed: pos.speed_longitude,
        });
    }

    Ok(positions)
}

/// Get heliocentric chart (planets only)
pub fn get_heliocentric_chart(jd: f64) -> Result<HeliocentricChart> {
    let planets = get_all_heliocentric_positions(jd)?;
    Ok(HeliocentricChart { planets })
}

/// Check if Moon is void-of-course
/// VoC = Moon won't make major aspects before leaving current sign
pub fn is_void_of_course_moon(jd: f64) -> Result<bool> {
    let positions = get_all_planetary_positions(jd)?;
    let moon = positions.iter().find(|p| p.planet_key == "moon").unwrap();

    let degrees_until_sign_change = 30.0 - moon.sign_degree;

    // Quick check: if very close to sign boundary with slow speed
    if degrees_until_sign_change < 2.0 && moon.speed.abs() < 0.4 {
        return Ok(true);
    }

    // Check for applying major aspects
    let major_aspects = [0.0, 60.0, 90.0, 120.0, 180.0];
    let orb = 8.0;

    for planet in &positions {
        if planet.planet_key == "moon" { continue; }

        let mut angle = (moon.longitude - planet.longitude).abs();
        if angle > 180.0 { angle = 360.0 - angle; }

        for &aspect_angle in &major_aspects {
            let diff = (angle - aspect_angle).abs();
            if diff < orb {
                // Moon is applying to an aspect, not VoC
                return Ok(false);
            }
        }
    }

    // In last 5 degrees with no aspects found
    Ok(degrees_until_sign_change < 5.0)
}

/// Get which house a planet occupies
pub fn get_planet_in_house(longitude: f64, house_cusps: &[HouseCusp]) -> u8 {
    for i in 0..12 {
        let current = &house_cusps[i];
        let next = &house_cusps[(i + 1) % 12];

        let current_cusp = current.cusp_longitude;
        let next_cusp = next.cusp_longitude;

        if current_cusp < next_cusp {
            if longitude >= current_cusp && longitude < next_cusp {
                return (i + 1) as u8;
            }
        } else {
            // House overlaps 0° (Aries point)
            if longitude >= current_cusp || longitude < next_cusp {
                return (i + 1) as u8;
            }
        }
    }
    1 // Fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sign_from_longitude() {
        assert_eq!(get_sign_from_longitude(0.0), "aries");
        assert_eq!(get_sign_from_longitude(30.0), "taurus");
        assert_eq!(get_sign_from_longitude(45.0), "taurus");
        assert_eq!(get_sign_from_longitude(90.0), "cancer");
        assert_eq!(get_sign_from_longitude(180.0), "libra");
        assert_eq!(get_sign_from_longitude(270.0), "capricorn");
        assert_eq!(get_sign_from_longitude(359.9), "pisces");
    }

    #[test]
    fn test_moon_phase() {
        assert_eq!(get_moon_phase(0.0, 0.0), MoonPhase::NewMoon);
        assert_eq!(get_moon_phase(0.0, 90.0), MoonPhase::FirstQuarter);
        assert_eq!(get_moon_phase(0.0, 180.0), MoonPhase::FullMoon);
        assert_eq!(get_moon_phase(0.0, 270.0), MoonPhase::LastQuarter);
    }

    #[test]
    fn test_planetary_hour() {
        // Sunday hour 0 (midnight) = Sun (day ruler)
        let ruler0 = get_planetary_hour_ruler(2000, 1, 2, 0); // Jan 2, 2000 = Sunday
        assert_eq!(ruler0, "sun");

        // Sunday hour 1 = Venus (next in Chaldean order after Sun)
        let ruler1 = get_planetary_hour_ruler(2000, 1, 2, 1);
        assert_eq!(ruler1, "venus");

        // Sunday hour 7 = Sun again (completes one cycle)
        let ruler7 = get_planetary_hour_ruler(2000, 1, 2, 7);
        assert_eq!(ruler7, "sun");
    }

    #[test]
    fn test_sign_degree() {
        assert!((get_sign_degree(45.0) - 15.0).abs() < 0.001);
        assert!((get_sign_degree(90.0) - 0.0).abs() < 0.001);
        assert!((get_sign_degree(100.0) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_orb_config_default() {
        let config = OrbConfig::default();
        assert_eq!(config.conjunction, 8.0);
        assert_eq!(config.opposition, 8.0);
        assert_eq!(config.square, 8.0);
        assert_eq!(config.trine, 8.0);
        assert_eq!(config.sextile, 6.0);
        assert_eq!(config.quincunx, 5.0);
        assert_eq!(config.semi_sextile, 4.0);
    }

    #[test]
    fn test_orb_config_get_orb() {
        let config = OrbConfig {
            conjunction: 10.0,
            opposition: 9.0,
            square: 8.0,
            trine: 7.0,
            sextile: 6.0,
            quincunx: 5.0,
            semi_sextile: 3.0,
            semi_square: 2.0,
            sesquiquadrate: 2.0,
            quintile: 1.0,
        };

        assert_eq!(config.get_orb(AspectType::Conjunction), 10.0);
        assert_eq!(config.get_orb(AspectType::Opposition), 9.0);
        assert_eq!(config.get_orb(AspectType::Sextile), 6.0);
        assert_eq!(config.get_orb(AspectType::Quintile), 1.0);
    }

    #[test]
    fn test_compute_aspects_with_custom_orbs() {
        // Create two positions 95 degrees apart (out of normal square range but in custom range)
        let pos1 = PlanetPosition {
            planet_key: "sun",
            longitude: 0.0,
            sign_key: "aries",
            sign_degree: 0.0,
            is_retrograde: false,
            speed: 1.0,
        };
        let pos2 = PlanetPosition {
            planet_key: "moon",
            longitude: 95.0, // 5° past exact square
            sign_key: "cancer",
            sign_degree: 5.0,
            is_retrograde: false,
            speed: 13.0,
        };

        let chart = vec![pos1, pos2];

        // With default orbs (8° for square), 95° should NOT be a square
        let narrow_orbs = OrbConfig {
            square: 4.0,
            ..Default::default()
        };
        let narrow_aspects = compute_aspects_with_orbs(&chart, &chart, &narrow_orbs);
        let has_square = narrow_aspects.iter().any(|a| a.aspect_type == AspectType::Square);
        assert!(!has_square, "Should not find square with 4° orb");

        // With wide orbs (10° for square), 95° SHOULD be a square
        let wide_orbs = OrbConfig {
            square: 10.0,
            ..Default::default()
        };
        let wide_aspects = compute_aspects_with_orbs(&chart, &chart, &wide_orbs);
        let has_square = wide_aspects.iter().any(|a| a.aspect_type == AspectType::Square);
        assert!(has_square, "Should find square with 10° orb");
    }
}
