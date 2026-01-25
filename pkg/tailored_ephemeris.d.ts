/* tslint:disable */
/* eslint-disable */

/**
 * Planet constants for JavaScript
 */
export class SE {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static readonly FLG_SPEED: number;
    static readonly GREG_CAL: number;
    static readonly JUL_CAL: number;
    static readonly JUPITER: number;
    static readonly MARS: number;
    static readonly MEAN_NODE: number;
    static readonly MERCURY: number;
    static readonly MOON: number;
    static readonly NEPTUNE: number;
    static readonly PLUTO: number;
    static readonly SATURN: number;
    static readonly SUN: number;
    static readonly TRUE_NODE: number;
    static readonly URANUS: number;
    static readonly VENUS: number;
}

/**
 * Calculate basic chart (sun, moon, rising signs)
 *
 * Quick calculation for sun sign, moon sign, and rising sign
 */
export function calculateChart(year: number, month: number, day: number, hour: number, lat: number, lon: number): any;

/**
 * Compute aspects within a single chart (mundane aspects)
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * Array of aspects between planets at that moment
 */
export function computeMundaneAspects(jd_ut: number): any;

/**
 * Compute aspects within a single chart (mundane aspects) with configurable orbs
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `orb_config` - Object with orb settings
 *
 * # Returns
 * Array of aspects between planets at that moment
 */
export function computeMundaneAspectsWithOrbs(jd_ut: number, orb_config: any): any;

/**
 * Compute natal chart aspects with configurable orbs
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `lat` - Geographic latitude
 * * `lon` - Geographic longitude
 * * `orb_config` - Object with orb settings
 *
 * # Returns
 * Array of aspects between natal planets
 */
export function computeNatalAspectsWithOrbs(jd_ut: number, lat: number, lon: number, orb_config: any): any;

/**
 * Compute aspects between transit and natal positions
 *
 * # Arguments
 * * `jd_transit` - Julian Day for transit positions
 * * `natal_positions` - JSON array of natal positions (from getNatalChart().planets)
 *
 * # Returns
 * Array of aspects found between transit and natal charts
 */
export function computeTransitAspects(jd_transit: number, natal_positions: any): any;

/**
 * Compute aspects between transit and natal positions with configurable orbs
 *
 * # Arguments
 * * `jd_transit` - Julian Day for transit positions
 * * `natal_positions` - JSON array of natal positions (from getNatalChart().planets)
 * * `orb_config` - Object with orb settings: { conjunction, opposition, square, trine, sextile, quincunx, semiSextile, semiSquare, sesquiquadrate, quintile }
 *
 * # Returns
 * Array of aspects found between transit and natal charts
 *
 * # Example orb_config
 * ```javascript
 * {
 *   conjunction: 8,
 *   opposition: 8,
 *   square: 8,
 *   trine: 8,
 *   sextile: 6,
 *   quincunx: 5,
 *   semiSextile: 4,
 *   semiSquare: 4,
 *   sesquiquadrate: 4,
 *   quintile: 4
 * }
 * ```
 */
export function computeTransitAspectsWithOrbs(jd_transit: number, natal_positions: any, orb_config: any): any;

/**
 * Get all planetary positions at a given time
 *
 * Returns array of planet positions with sign, degree, and retrograde status
 */
export function getAllPlanetaryPositions(jd_ut: number): any;

/**
 * Calculate moon phase
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * Moon phase key: "new_moon", "waxing_crescent", etc.
 */
export function getMoonPhase(jd_ut: number): string;

/**
 * Get complete natal chart
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `lat` - Geographic latitude
 * * `lon` - Geographic longitude
 *
 * # Returns
 * Complete natal chart with planets, houses, angles, and north node
 */
export function getNatalChart(jd_ut: number, lat: number, lon: number): any;

/**
 * Get planet's house placement
 *
 * # Arguments
 * * `planet_longitude` - Planet's ecliptic longitude
 * * `house_cusps` - Array of 12 house cusp longitudes
 *
 * # Returns
 * House number (1-12)
 */
export function getPlanetInHouse(planet_longitude: number, house_cusps: Float64Array): number;

/**
 * Get planetary hour ruler
 *
 * # Arguments
 * * `year` - Year
 * * `month` - Month (1-12)
 * * `day` - Day of month
 * * `hour` - Hour (0-23)
 *
 * # Returns
 * Planet key ruling that hour: "sun", "moon", "mars", etc.
 */
export function getPlanetaryHourRuler(year: number, month: number, day: number, hour: number): string;

/**
 * Get zodiac sign from longitude
 *
 * # Arguments
 * * `longitude` - Ecliptic longitude (0-360)
 *
 * # Returns
 * Sign key: "aries", "taurus", etc.
 */
export function getSignFromLongitude(longitude: number): string;

/**
 * Check if Moon is void-of-course
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * true if Moon is void-of-course
 */
export function isVoidOfCourseMoon(jd_ut: number): boolean;

/**
 * Calculate planet position
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `ipl` - Planet number (0=Sun, 1=Moon, 2=Mercury, ..., 11=True Node)
 * * `iflag` - Calculation flags (256 = SEFLG_SPEED for speed calculation)
 *
 * # Returns
 * Position object with longitude, latitude, distance, longitudeSpeed
 */
export function swe_calc_ut(jd_ut: number, ipl: number, iflag?: number | null): any;

/**
 * Calculate all planets at once
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `iflag` - Calculation flags
 *
 * # Returns
 * Object with all planet positions
 */
export function swe_calc_ut_all(jd_ut: number, iflag?: number | null): any;

/**
 * Batch calculation for multiple planets
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `planets` - Array of planet numbers
 * * `iflag` - Calculation flags
 *
 * # Returns
 * Object mapping planet numbers to position objects
 */
export function swe_calc_ut_batch(jd_ut: number, planets: Int32Array, iflag?: number | null): any;

/**
 * Get planet name
 */
export function swe_get_planet_name(ipl: number): string;

/**
 * Calculate house cusps (Placidus system)
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `lat` - Geographic latitude in degrees
 * * `lon` - Geographic longitude in degrees
 * * `hsys` - House system (only 'P' for Placidus supported, ignored)
 *
 * # Returns
 * Object with cusps array (12 elements), ascendant, mc, armc, vertex
 */
export function swe_houses(jd_ut: number, lat: number, lon: number, _hsys?: string | null): any;

/**
 * Calculate Julian Day from calendar date
 *
 * # Arguments
 * * `year` - Year (negative for BC)
 * * `month` - Month (1-12)
 * * `day` - Day of month
 * * `hour` - Hour as decimal (0.0-24.0)
 * * `gregflag` - Calendar flag: 1 for Gregorian (default), 0 for Julian
 *
 * # Returns
 * Julian Day number
 */
export function swe_julday(year: number, month: number, day: number, hour: number, gregflag?: number | null): number;

/**
 * Convert Julian Day to calendar date
 *
 * # Arguments
 * * `jd` - Julian Day number
 * * `gregflag` - Calendar flag: 1 for Gregorian (default), 0 for Julian
 *
 * # Returns
 * Object with year, month, day, hour
 */
export function swe_revjul(jd: number, gregflag?: number | null): any;
