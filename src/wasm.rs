//! WebAssembly bindings for horoscope calculations
//!
//! Provides a JavaScript-compatible API matching sweph-wasm interface.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{calc_ut, calc_houses, julian, Planet, Position, Houses, constants, astrology};

/// Planet position result for JavaScript
#[derive(Serialize, Deserialize)]
pub struct JsPosition {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    #[serde(rename = "longitudeSpeed")]
    pub longitude_speed: f64,
}

impl From<Position> for JsPosition {
    fn from(p: Position) -> Self {
        JsPosition {
            longitude: p.longitude,
            latitude: p.latitude,
            distance: p.distance,
            longitude_speed: p.speed_longitude,
        }
    }
}

/// House calculation result for JavaScript
#[derive(Serialize, Deserialize)]
pub struct JsHouses {
    pub cusps: Vec<f64>,
    pub ascendant: f64,
    pub mc: f64,
    pub armc: f64,
    pub vertex: f64,
}

impl From<Houses> for JsHouses {
    fn from(h: Houses) -> Self {
        JsHouses {
            cusps: h.cusps[1..=12].to_vec(),
            ascendant: h.ascendant,
            mc: h.mc,
            armc: h.armc,
            vertex: h.vertex,
        }
    }
}

/// Calculate Julian Day from calendar date
///
/// # Arguments
/// * `year` - Year (negative for BC)
/// * `month` - Month (1-12)
/// * `day` - Day of month
/// * `hour` - Hour as decimal (0.0-24.0)
/// * `gregflag` - Calendar flag: 1 for Gregorian (default), 0 for Julian
///
/// # Returns
/// Julian Day number
#[wasm_bindgen(js_name = swe_julday)]
pub fn swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: Option<i32>) -> f64 {
    let flag = gregflag.unwrap_or(constants::SE_GREG_CAL);
    julian::julday(year, month, day, hour, flag)
}

/// Convert Julian Day to calendar date
///
/// # Arguments
/// * `jd` - Julian Day number
/// * `gregflag` - Calendar flag: 1 for Gregorian (default), 0 for Julian
///
/// # Returns
/// Object with year, month, day, hour
#[wasm_bindgen(js_name = swe_revjul)]
pub fn swe_revjul(jd: f64, gregflag: Option<i32>) -> JsValue {
    let flag = gregflag.unwrap_or(constants::SE_GREG_CAL);
    let (year, month, day, hour) = julian::revjul(jd, flag);

    let result = serde_json::json!({
        "year": year,
        "month": month,
        "day": day,
        "hour": hour
    });

    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Calculate planet position
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `ipl` - Planet number (0=Sun, 1=Moon, 2=Mercury, ..., 11=True Node)
/// * `iflag` - Calculation flags (256 = SEFLG_SPEED for speed calculation)
///
/// # Returns
/// Position object with longitude, latitude, distance, longitudeSpeed
#[wasm_bindgen(js_name = swe_calc_ut)]
pub fn swe_calc_ut(jd_ut: f64, ipl: i32, iflag: Option<i32>) -> JsValue {
    let planet = match Planet::from_i32(ipl) {
        Some(p) => p,
        None => return JsValue::NULL,
    };

    let flags = iflag.unwrap_or(0);
    let calc_speed = (flags & constants::SEFLG_SPEED) != 0;

    match calc_ut(jd_ut, planet, calc_speed) {
        Ok(pos) => {
            let js_pos: JsPosition = pos.into();
            serde_wasm_bindgen::to_value(&js_pos).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Calculate house cusps (Placidus system)
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `lat` - Geographic latitude in degrees
/// * `lon` - Geographic longitude in degrees
/// * `hsys` - House system (only 'P' for Placidus supported, ignored)
///
/// # Returns
/// Object with cusps array (12 elements), ascendant, mc, armc, vertex
#[wasm_bindgen(js_name = swe_houses)]
pub fn swe_houses(jd_ut: f64, lat: f64, lon: f64, _hsys: Option<String>) -> JsValue {
    // Note: We only support Placidus, hsys parameter is ignored
    match calc_houses(jd_ut, lat, lon) {
        Ok(houses) => {
            let js_houses: JsHouses = houses.into();
            serde_wasm_bindgen::to_value(&js_houses).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Get planet name
#[wasm_bindgen(js_name = swe_get_planet_name)]
pub fn swe_get_planet_name(ipl: i32) -> String {
    match ipl {
        0 => "Sun".to_string(),
        1 => "Moon".to_string(),
        2 => "Mercury".to_string(),
        3 => "Venus".to_string(),
        4 => "Mars".to_string(),
        5 => "Jupiter".to_string(),
        6 => "Saturn".to_string(),
        7 => "Uranus".to_string(),
        8 => "Neptune".to_string(),
        9 => "Pluto".to_string(),
        10 => "Mean Node".to_string(),
        11 => "True Node".to_string(),
        14 => "Earth".to_string(),
        _ => format!("Planet {}", ipl),
    }
}

/// Planet constants for JavaScript
#[wasm_bindgen]
pub struct SE {
    _private: (),
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl SE {
    #[wasm_bindgen(getter)]
    pub fn SUN() -> i32 { 0 }

    #[wasm_bindgen(getter)]
    pub fn MOON() -> i32 { 1 }

    #[wasm_bindgen(getter)]
    pub fn MERCURY() -> i32 { 2 }

    #[wasm_bindgen(getter)]
    pub fn VENUS() -> i32 { 3 }

    #[wasm_bindgen(getter)]
    pub fn MARS() -> i32 { 4 }

    #[wasm_bindgen(getter)]
    pub fn JUPITER() -> i32 { 5 }

    #[wasm_bindgen(getter)]
    pub fn SATURN() -> i32 { 6 }

    #[wasm_bindgen(getter)]
    pub fn URANUS() -> i32 { 7 }

    #[wasm_bindgen(getter)]
    pub fn NEPTUNE() -> i32 { 8 }

    #[wasm_bindgen(getter)]
    pub fn PLUTO() -> i32 { 9 }

    #[wasm_bindgen(getter)]
    pub fn MEAN_NODE() -> i32 { 10 }

    #[wasm_bindgen(getter)]
    pub fn TRUE_NODE() -> i32 { 11 }

    #[wasm_bindgen(getter)]
    pub fn EARTH() -> i32 { 14 }

    #[wasm_bindgen(getter)]
    pub fn GREG_CAL() -> i32 { 1 }

    #[wasm_bindgen(getter)]
    pub fn JUL_CAL() -> i32 { 0 }

    #[wasm_bindgen(getter)]
    pub fn FLG_SPEED() -> i32 { 256 }
}

/// Batch calculation for multiple planets
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `planets` - Array of planet numbers
/// * `iflag` - Calculation flags
///
/// # Returns
/// Object mapping planet numbers to position objects
#[wasm_bindgen(js_name = swe_calc_ut_batch)]
pub fn swe_calc_ut_batch(jd_ut: f64, planets: &[i32], iflag: Option<i32>) -> JsValue {
    use std::collections::HashMap;

    let flags = iflag.unwrap_or(0);
    let calc_speed = (flags & constants::SEFLG_SPEED) != 0;

    let mut results: HashMap<i32, JsPosition> = HashMap::new();

    for &ipl in planets {
        if let Some(planet) = Planet::from_i32(ipl) {
            if let Ok(pos) = calc_ut(jd_ut, planet, calc_speed) {
                results.insert(ipl, pos.into());
            }
        }
    }

    serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
}

/// Calculate all planets at once
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `iflag` - Calculation flags
///
/// # Returns
/// Object with all planet positions
#[wasm_bindgen(js_name = swe_calc_ut_all)]
pub fn swe_calc_ut_all(jd_ut: f64, iflag: Option<i32>) -> JsValue {
    let all_planets = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11]; // All planets + True Node
    swe_calc_ut_batch(jd_ut, &all_planets, iflag)
}

// ============================================================================
// High-Level Astrological Functions
// These return ready-to-use data structures
// ============================================================================

/// Planet position with derived data for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsPlanetPosition {
    pub planet_key: String,
    pub longitude: f64,
    pub sign_key: String,
    pub sign_degree: f64,
    pub is_retrograde: bool,
    pub speed: f64,
}

/// House cusp with derived data for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsHouseCusp {
    pub house_number: u8,
    pub cusp_longitude: f64,
    pub sign_key: String,
    pub sign_degree: f64,
}

/// Complete natal chart for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsNatalChart {
    pub planets: Vec<JsPlanetPosition>,
    pub houses: Vec<JsHouseCusp>,
    pub ascendant: f64,
    pub midheaven: f64,
    pub north_node: JsNorthNode,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsNorthNode {
    pub longitude: f64,
    pub sign_key: String,
    pub sign_degree: f64,
}

/// Computed aspect for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsAspect {
    pub planet1_key: String,
    pub planet2_key: String,
    pub aspect_key: String,
    pub orb: f64,
    pub is_applying: bool,
}

/// Orb configuration for JavaScript
/// Pass this to aspect calculation functions to use custom orbs
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JsOrbConfig {
    #[serde(default = "default_major_orb")]
    pub conjunction: f64,
    #[serde(default = "default_major_orb")]
    pub opposition: f64,
    #[serde(default = "default_major_orb")]
    pub square: f64,
    #[serde(default = "default_major_orb")]
    pub trine: f64,
    #[serde(default = "default_sextile_orb")]
    pub sextile: f64,
    #[serde(default = "default_quincunx_orb")]
    pub quincunx: f64,
    #[serde(default = "default_minor_orb")]
    pub semi_sextile: f64,
    #[serde(default = "default_minor_orb")]
    pub semi_square: f64,
    #[serde(default = "default_minor_orb")]
    pub sesquiquadrate: f64,
    #[serde(default = "default_minor_orb")]
    pub quintile: f64,
}

fn default_major_orb() -> f64 { 8.0 }
fn default_sextile_orb() -> f64 { 6.0 }
fn default_quincunx_orb() -> f64 { 5.0 }
fn default_minor_orb() -> f64 { 4.0 }

impl From<JsOrbConfig> for astrology::OrbConfig {
    fn from(js: JsOrbConfig) -> Self {
        astrology::OrbConfig {
            conjunction: js.conjunction,
            opposition: js.opposition,
            square: js.square,
            trine: js.trine,
            sextile: js.sextile,
            quincunx: js.quincunx,
            semi_sextile: js.semi_sextile,
            semi_square: js.semi_square,
            sesquiquadrate: js.sesquiquadrate,
            quintile: js.quintile,
        }
    }
}

/// Heliocentric chart for JavaScript (planets only, no houses/angles)
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsHeliocentricChart {
    pub planets: Vec<JsPlanetPosition>,
}

/// Get all planetary positions at a given time
///
/// Returns array of planet positions with sign, degree, and retrograde status
#[wasm_bindgen(js_name = getAllPlanetaryPositions)]
pub fn get_all_planetary_positions(jd_ut: f64) -> JsValue {
    match astrology::get_all_planetary_positions(jd_ut) {
        Ok(positions) => {
            let js_positions: Vec<JsPlanetPosition> = positions.iter().map(|p| {
                JsPlanetPosition {
                    planet_key: p.planet_key.to_string(),
                    longitude: p.longitude,
                    sign_key: p.sign_key.to_string(),
                    sign_degree: p.sign_degree,
                    is_retrograde: p.is_retrograde,
                    speed: p.speed,
                }
            }).collect();
            serde_wasm_bindgen::to_value(&js_positions).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Get complete natal chart
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `lat` - Geographic latitude
/// * `lon` - Geographic longitude
///
/// # Returns
/// Complete natal chart with planets, houses, angles, and north node
#[wasm_bindgen(js_name = getNatalChart)]
pub fn get_natal_chart(jd_ut: f64, lat: f64, lon: f64) -> JsValue {
    match astrology::get_natal_chart(jd_ut, lat, lon) {
        Ok(chart) => {
            let js_chart = JsNatalChart {
                planets: chart.planets.iter().map(|p| JsPlanetPosition {
                    planet_key: p.planet_key.to_string(),
                    longitude: p.longitude,
                    sign_key: p.sign_key.to_string(),
                    sign_degree: p.sign_degree,
                    is_retrograde: p.is_retrograde,
                    speed: p.speed,
                }).collect(),
                houses: chart.houses.iter().map(|h| JsHouseCusp {
                    house_number: h.house_number,
                    cusp_longitude: h.cusp_longitude,
                    sign_key: h.sign_key.to_string(),
                    sign_degree: h.sign_degree,
                }).collect(),
                ascendant: chart.ascendant,
                midheaven: chart.midheaven,
                north_node: JsNorthNode {
                    longitude: chart.north_node_longitude,
                    sign_key: chart.north_node_sign.to_string(),
                    sign_degree: chart.north_node_degree,
                },
            };
            serde_wasm_bindgen::to_value(&js_chart).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Get heliocentric planetary positions at a given time
///
/// Returns array of planet positions for Earth + Mercury through Pluto (9 planets).
/// All positions are heliocentric (relative to the Sun).
/// isRetrograde is always false (no retrograde in heliocentric frame).
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
///
/// # Returns
/// Array of planet positions with sign, degree, speed, and isRetrograde (always false)
#[wasm_bindgen(js_name = getHeliocentricPositions)]
pub fn get_heliocentric_positions(jd_ut: f64) -> JsValue {
    match astrology::get_all_heliocentric_positions(jd_ut) {
        Ok(positions) => {
            let js_positions: Vec<JsPlanetPosition> = positions.iter().map(|p| {
                JsPlanetPosition {
                    planet_key: p.planet_key.to_string(),
                    longitude: p.longitude,
                    sign_key: p.sign_key.to_string(),
                    sign_degree: p.sign_degree,
                    is_retrograde: false,
                    speed: p.speed,
                }
            }).collect();
            serde_wasm_bindgen::to_value(&js_positions).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Get heliocentric chart
///
/// Returns a chart with only planets (no houses, ascendant, or midheaven
/// since those are geocentric concepts).
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
///
/// # Returns
/// Heliocentric chart with planets array
#[wasm_bindgen(js_name = getHeliocentricChart)]
pub fn get_heliocentric_chart(jd_ut: f64) -> JsValue {
    match astrology::get_heliocentric_chart(jd_ut) {
        Ok(chart) => {
            let js_chart = JsHeliocentricChart {
                planets: chart.planets.iter().map(|p| JsPlanetPosition {
                    planet_key: p.planet_key.to_string(),
                    longitude: p.longitude,
                    sign_key: p.sign_key.to_string(),
                    sign_degree: p.sign_degree,
                    is_retrograde: false,
                    speed: p.speed,
                }).collect(),
            };
            serde_wasm_bindgen::to_value(&js_chart).unwrap_or(JsValue::NULL)
        }
        Err(_) => JsValue::NULL,
    }
}

/// Calculate moon phase
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
///
/// # Returns
/// Moon phase key: "new_moon", "waxing_crescent", etc.
#[wasm_bindgen(js_name = getMoonPhase)]
pub fn get_moon_phase(jd_ut: f64) -> String {
    match astrology::calculate_moon_phase(jd_ut) {
        Ok(phase) => phase.as_str().to_string(),
        Err(_) => "unknown".to_string(),
    }
}

/// Get zodiac sign from longitude
///
/// # Arguments
/// * `longitude` - Ecliptic longitude (0-360)
///
/// # Returns
/// Sign key: "aries", "taurus", etc.
#[wasm_bindgen(js_name = getSignFromLongitude)]
pub fn get_sign_from_longitude(longitude: f64) -> String {
    astrology::get_sign_from_longitude(longitude).to_string()
}

/// Check if Moon is void-of-course
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
///
/// # Returns
/// true if Moon is void-of-course
#[wasm_bindgen(js_name = isVoidOfCourseMoon)]
pub fn is_void_of_course_moon(jd_ut: f64) -> bool {
    astrology::is_void_of_course_moon(jd_ut).unwrap_or(false)
}

/// Get planetary hour ruler
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
/// * `hour` - Hour (0-23)
///
/// # Returns
/// Planet key ruling that hour: "sun", "moon", "mars", etc.
#[wasm_bindgen(js_name = getPlanetaryHourRuler)]
pub fn get_planetary_hour_ruler(year: i32, month: i32, day: i32, hour: u32) -> String {
    astrology::get_planetary_hour_ruler(year, month, day, hour).to_string()
}

/// Compute aspects between transit and natal positions
///
/// # Arguments
/// * `jd_transit` - Julian Day for transit positions
/// * `natal_positions` - JSON array of natal positions (from getNatalChart().planets)
///
/// # Returns
/// Array of aspects found between transit and natal charts
#[wasm_bindgen(js_name = computeTransitAspects)]
pub fn compute_transit_aspects(jd_transit: f64, natal_positions: JsValue) -> JsValue {
    // Get current transit positions
    let transit_positions = match astrology::get_all_planetary_positions(jd_transit) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };

    // Parse natal positions from JS
    let natal: Vec<JsPlanetPosition> = match serde_wasm_bindgen::from_value(natal_positions) {
        Ok(n) => n,
        Err(_) => return JsValue::NULL,
    };

    // Convert to internal format
    let natal_internal: Vec<astrology::PlanetPosition> = natal.iter().map(|p| {
        astrology::PlanetPosition {
            planet_key: match p.planet_key.as_str() {
                "sun" => "sun",
                "moon" => "moon",
                "mercury" => "mercury",
                "venus" => "venus",
                "mars" => "mars",
                "jupiter" => "jupiter",
                "saturn" => "saturn",
                "uranus" => "uranus",
                "neptune" => "neptune",
                "pluto" => "pluto",
                _ => "sun",
            },
            longitude: p.longitude,
            sign_key: match p.sign_key.as_str() {
                "aries" => "aries",
                "taurus" => "taurus",
                "gemini" => "gemini",
                "cancer" => "cancer",
                "leo" => "leo",
                "virgo" => "virgo",
                "libra" => "libra",
                "scorpio" => "scorpio",
                "sagittarius" => "sagittarius",
                "capricorn" => "capricorn",
                "aquarius" => "aquarius",
                "pisces" => "pisces",
                _ => "aries",
            },
            sign_degree: p.sign_degree,
            is_retrograde: p.is_retrograde,
            speed: p.speed,
        }
    }).collect();

    // Compute aspects
    let aspects = astrology::compute_aspects(&transit_positions, &natal_internal);

    // Convert to JS format
    let js_aspects: Vec<JsAspect> = aspects.iter().map(|a| JsAspect {
        planet1_key: a.planet1_key.to_string(),
        planet2_key: a.planet2_key.to_string(),
        aspect_key: a.aspect_type.as_str().to_string(),
        orb: a.orb,
        is_applying: a.is_applying,
    }).collect();

    serde_wasm_bindgen::to_value(&js_aspects).unwrap_or(JsValue::NULL)
}

/// Compute aspects within a single chart (mundane aspects)
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
///
/// # Returns
/// Array of aspects between planets at that moment
#[wasm_bindgen(js_name = computeMundaneAspects)]
pub fn compute_mundane_aspects(jd_ut: f64) -> JsValue {
    let positions = match astrology::get_all_planetary_positions(jd_ut) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };

    let aspects = astrology::compute_aspects(&positions, &positions);

    let js_aspects: Vec<JsAspect> = aspects.iter().map(|a| JsAspect {
        planet1_key: a.planet1_key.to_string(),
        planet2_key: a.planet2_key.to_string(),
        aspect_key: a.aspect_type.as_str().to_string(),
        orb: a.orb,
        is_applying: a.is_applying,
    }).collect();

    serde_wasm_bindgen::to_value(&js_aspects).unwrap_or(JsValue::NULL)
}

/// Compute aspects between transit and natal positions with configurable orbs
///
/// # Arguments
/// * `jd_transit` - Julian Day for transit positions
/// * `natal_positions` - JSON array of natal positions (from getNatalChart().planets)
/// * `orb_config` - Object with orb settings: { conjunction, opposition, square, trine, sextile, quincunx, semiSextile, semiSquare, sesquiquadrate, quintile }
///
/// # Returns
/// Array of aspects found between transit and natal charts
///
/// # Example orb_config
/// ```javascript
/// {
///   conjunction: 8,
///   opposition: 8,
///   square: 8,
///   trine: 8,
///   sextile: 6,
///   quincunx: 5,
///   semiSextile: 4,
///   semiSquare: 4,
///   sesquiquadrate: 4,
///   quintile: 4
/// }
/// ```
#[wasm_bindgen(js_name = computeTransitAspectsWithOrbs)]
pub fn compute_transit_aspects_with_orbs(jd_transit: f64, natal_positions: JsValue, orb_config: JsValue) -> JsValue {
    // Get current transit positions
    let transit_positions = match astrology::get_all_planetary_positions(jd_transit) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };

    // Parse natal positions from JS
    let natal: Vec<JsPlanetPosition> = match serde_wasm_bindgen::from_value(natal_positions) {
        Ok(n) => n,
        Err(_) => return JsValue::NULL,
    };

    // Parse orb config from JS
    let js_orbs: JsOrbConfig = match serde_wasm_bindgen::from_value(orb_config) {
        Ok(o) => o,
        Err(_) => JsOrbConfig::default(),
    };
    let orbs: astrology::OrbConfig = js_orbs.into();

    // Convert to internal format
    let natal_internal: Vec<astrology::PlanetPosition> = natal.iter().map(|p| {
        astrology::PlanetPosition {
            planet_key: match p.planet_key.as_str() {
                "sun" => "sun",
                "moon" => "moon",
                "mercury" => "mercury",
                "venus" => "venus",
                "mars" => "mars",
                "jupiter" => "jupiter",
                "saturn" => "saturn",
                "uranus" => "uranus",
                "neptune" => "neptune",
                "pluto" => "pluto",
                _ => "sun",
            },
            longitude: p.longitude,
            sign_key: match p.sign_key.as_str() {
                "aries" => "aries",
                "taurus" => "taurus",
                "gemini" => "gemini",
                "cancer" => "cancer",
                "leo" => "leo",
                "virgo" => "virgo",
                "libra" => "libra",
                "scorpio" => "scorpio",
                "sagittarius" => "sagittarius",
                "capricorn" => "capricorn",
                "aquarius" => "aquarius",
                "pisces" => "pisces",
                _ => "aries",
            },
            sign_degree: p.sign_degree,
            is_retrograde: p.is_retrograde,
            speed: p.speed,
        }
    }).collect();

    // Compute aspects with custom orbs
    let aspects = astrology::compute_aspects_with_orbs(&transit_positions, &natal_internal, &orbs);

    // Convert to JS format
    let js_aspects: Vec<JsAspect> = aspects.iter().map(|a| JsAspect {
        planet1_key: a.planet1_key.to_string(),
        planet2_key: a.planet2_key.to_string(),
        aspect_key: a.aspect_type.as_str().to_string(),
        orb: a.orb,
        is_applying: a.is_applying,
    }).collect();

    serde_wasm_bindgen::to_value(&js_aspects).unwrap_or(JsValue::NULL)
}

/// Compute aspects within a single chart (mundane aspects) with configurable orbs
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `orb_config` - Object with orb settings
///
/// # Returns
/// Array of aspects between planets at that moment
#[wasm_bindgen(js_name = computeMundaneAspectsWithOrbs)]
pub fn compute_mundane_aspects_with_orbs(jd_ut: f64, orb_config: JsValue) -> JsValue {
    let positions = match astrology::get_all_planetary_positions(jd_ut) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };

    // Parse orb config from JS
    let js_orbs: JsOrbConfig = match serde_wasm_bindgen::from_value(orb_config) {
        Ok(o) => o,
        Err(_) => JsOrbConfig::default(),
    };
    let orbs: astrology::OrbConfig = js_orbs.into();

    let aspects = astrology::compute_aspects_with_orbs(&positions, &positions, &orbs);

    let js_aspects: Vec<JsAspect> = aspects.iter().map(|a| JsAspect {
        planet1_key: a.planet1_key.to_string(),
        planet2_key: a.planet2_key.to_string(),
        aspect_key: a.aspect_type.as_str().to_string(),
        orb: a.orb,
        is_applying: a.is_applying,
    }).collect();

    serde_wasm_bindgen::to_value(&js_aspects).unwrap_or(JsValue::NULL)
}

/// Compute natal chart aspects with configurable orbs
///
/// # Arguments
/// * `jd_ut` - Julian Day in Universal Time
/// * `lat` - Geographic latitude
/// * `lon` - Geographic longitude
/// * `orb_config` - Object with orb settings
///
/// # Returns
/// Array of aspects between natal planets
#[wasm_bindgen(js_name = computeNatalAspectsWithOrbs)]
pub fn compute_natal_aspects_with_orbs(jd_ut: f64, lat: f64, lon: f64, orb_config: JsValue) -> JsValue {
    let chart = match astrology::get_natal_chart(jd_ut, lat, lon) {
        Ok(c) => c,
        Err(_) => return JsValue::NULL,
    };

    // Parse orb config from JS
    let js_orbs: JsOrbConfig = match serde_wasm_bindgen::from_value(orb_config) {
        Ok(o) => o,
        Err(_) => JsOrbConfig::default(),
    };
    let orbs: astrology::OrbConfig = js_orbs.into();

    let aspects = astrology::compute_aspects_with_orbs(&chart.planets, &chart.planets, &orbs);

    let js_aspects: Vec<JsAspect> = aspects.iter().map(|a| JsAspect {
        planet1_key: a.planet1_key.to_string(),
        planet2_key: a.planet2_key.to_string(),
        aspect_key: a.aspect_type.as_str().to_string(),
        orb: a.orb,
        is_applying: a.is_applying,
    }).collect();

    serde_wasm_bindgen::to_value(&js_aspects).unwrap_or(JsValue::NULL)
}

/// Basic chart result for JavaScript
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsBasicChart {
    pub sun_sign: String,
    pub moon_sign: String,
    pub rising_sign: String,
}

/// Calculate basic chart (sun, moon, rising signs)
///
/// Quick calculation for sun sign, moon sign, and rising sign
#[wasm_bindgen(js_name = calculateChart)]
pub fn calculate_chart(year: i32, month: i32, day: i32, hour: f64, lat: f64, lon: f64) -> JsValue {
    let jd = julian::julday(year, month, day, hour, 1);

    let sun = match calc_ut(jd, Planet::Sun, false) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };
    let moon = match calc_ut(jd, Planet::Moon, false) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };
    let houses = match calc_houses(jd, lat, lon) {
        Ok(h) => h,
        Err(_) => return JsValue::NULL,
    };

    let result = JsBasicChart {
        sun_sign: astrology::get_sign_from_longitude(sun.longitude).to_string(),
        moon_sign: astrology::get_sign_from_longitude(moon.longitude).to_string(),
        rising_sign: astrology::get_sign_from_longitude(houses.ascendant).to_string(),
    };

    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get planet's house placement
///
/// # Arguments
/// * `planet_longitude` - Planet's ecliptic longitude
/// * `house_cusps` - Array of 12 house cusp longitudes
///
/// # Returns
/// House number (1-12)
#[wasm_bindgen(js_name = getPlanetInHouse)]
pub fn get_planet_in_house(planet_longitude: f64, house_cusps: &[f64]) -> u8 {
    if house_cusps.len() != 12 {
        return 1;
    }

    let cusps: Vec<astrology::HouseCusp> = house_cusps.iter().enumerate().map(|(i, &lon)| {
        astrology::HouseCusp {
            house_number: (i + 1) as u8,
            cusp_longitude: lon,
            sign_key: astrology::get_sign_from_longitude(lon),
            sign_degree: astrology::get_sign_degree(lon),
        }
    }).collect();

    astrology::get_planet_in_house(planet_longitude, &cusps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julday() {
        let jd = swe_julday(2000, 1, 1, 12.0, Some(1));
        assert!((jd - 2451545.0).abs() < 0.0001);
    }
}
