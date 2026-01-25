/* @ts-self-types="./tailored_ephemeris.d.ts" */

/**
 * Planet constants for JavaScript
 */
export class SE {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SEFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_se_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    static get FLG_SPEED() {
        const ret = wasm.se_FLG_SPEED();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get GREG_CAL() {
        const ret = wasm.se_GREG_CAL();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get JUL_CAL() {
        const ret = wasm.se_JUL_CAL();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get JUPITER() {
        const ret = wasm.se_JUPITER();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get MARS() {
        const ret = wasm.se_MARS();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get MEAN_NODE() {
        const ret = wasm.se_MEAN_NODE();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get MERCURY() {
        const ret = wasm.se_MERCURY();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get MOON() {
        const ret = wasm.se_GREG_CAL();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get NEPTUNE() {
        const ret = wasm.se_NEPTUNE();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get PLUTO() {
        const ret = wasm.se_PLUTO();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get SATURN() {
        const ret = wasm.se_SATURN();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get SUN() {
        const ret = wasm.se_JUL_CAL();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get TRUE_NODE() {
        const ret = wasm.se_TRUE_NODE();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get URANUS() {
        const ret = wasm.se_URANUS();
        return ret;
    }
    /**
     * @returns {number}
     */
    static get VENUS() {
        const ret = wasm.se_VENUS();
        return ret;
    }
}
if (Symbol.dispose) SE.prototype[Symbol.dispose] = SE.prototype.free;

/**
 * Calculate basic chart (sun, moon, rising signs)
 *
 * Quick calculation for sun sign, moon sign, and rising sign
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number} lat
 * @param {number} lon
 * @returns {any}
 */
export function calculateChart(year, month, day, hour, lat, lon) {
    const ret = wasm.calculateChart(year, month, day, hour, lat, lon);
    return ret;
}

/**
 * Compute aspects within a single chart (mundane aspects)
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * Array of aspects between planets at that moment
 * @param {number} jd_ut
 * @returns {any}
 */
export function computeMundaneAspects(jd_ut) {
    const ret = wasm.computeMundaneAspects(jd_ut);
    return ret;
}

/**
 * Compute aspects within a single chart (mundane aspects) with configurable orbs
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `orb_config` - Object with orb settings
 *
 * # Returns
 * Array of aspects between planets at that moment
 * @param {number} jd_ut
 * @param {any} orb_config
 * @returns {any}
 */
export function computeMundaneAspectsWithOrbs(jd_ut, orb_config) {
    const ret = wasm.computeMundaneAspectsWithOrbs(jd_ut, orb_config);
    return ret;
}

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
 * @param {number} jd_ut
 * @param {number} lat
 * @param {number} lon
 * @param {any} orb_config
 * @returns {any}
 */
export function computeNatalAspectsWithOrbs(jd_ut, lat, lon, orb_config) {
    const ret = wasm.computeNatalAspectsWithOrbs(jd_ut, lat, lon, orb_config);
    return ret;
}

/**
 * Compute aspects between transit and natal positions
 *
 * # Arguments
 * * `jd_transit` - Julian Day for transit positions
 * * `natal_positions` - JSON array of natal positions (from getNatalChart().planets)
 *
 * # Returns
 * Array of aspects found between transit and natal charts
 * @param {number} jd_transit
 * @param {any} natal_positions
 * @returns {any}
 */
export function computeTransitAspects(jd_transit, natal_positions) {
    const ret = wasm.computeTransitAspects(jd_transit, natal_positions);
    return ret;
}

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
 * @param {number} jd_transit
 * @param {any} natal_positions
 * @param {any} orb_config
 * @returns {any}
 */
export function computeTransitAspectsWithOrbs(jd_transit, natal_positions, orb_config) {
    const ret = wasm.computeTransitAspectsWithOrbs(jd_transit, natal_positions, orb_config);
    return ret;
}

/**
 * Get all planetary positions at a given time
 *
 * Returns array of planet positions with sign, degree, and retrograde status
 * @param {number} jd_ut
 * @returns {any}
 */
export function getAllPlanetaryPositions(jd_ut) {
    const ret = wasm.getAllPlanetaryPositions(jd_ut);
    return ret;
}

/**
 * Calculate moon phase
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * Moon phase key: "new_moon", "waxing_crescent", etc.
 * @param {number} jd_ut
 * @returns {string}
 */
export function getMoonPhase(jd_ut) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.getMoonPhase(jd_ut);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

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
 * @param {number} jd_ut
 * @param {number} lat
 * @param {number} lon
 * @returns {any}
 */
export function getNatalChart(jd_ut, lat, lon) {
    const ret = wasm.getNatalChart(jd_ut, lat, lon);
    return ret;
}

/**
 * Get planet's house placement
 *
 * # Arguments
 * * `planet_longitude` - Planet's ecliptic longitude
 * * `house_cusps` - Array of 12 house cusp longitudes
 *
 * # Returns
 * House number (1-12)
 * @param {number} planet_longitude
 * @param {Float64Array} house_cusps
 * @returns {number}
 */
export function getPlanetInHouse(planet_longitude, house_cusps) {
    const ptr0 = passArrayF64ToWasm0(house_cusps, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.getPlanetInHouse(planet_longitude, ptr0, len0);
    return ret;
}

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
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @returns {string}
 */
export function getPlanetaryHourRuler(year, month, day, hour) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.getPlanetaryHourRuler(year, month, day, hour);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Get zodiac sign from longitude
 *
 * # Arguments
 * * `longitude` - Ecliptic longitude (0-360)
 *
 * # Returns
 * Sign key: "aries", "taurus", etc.
 * @param {number} longitude
 * @returns {string}
 */
export function getSignFromLongitude(longitude) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.getSignFromLongitude(longitude);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Check if Moon is void-of-course
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 *
 * # Returns
 * true if Moon is void-of-course
 * @param {number} jd_ut
 * @returns {boolean}
 */
export function isVoidOfCourseMoon(jd_ut) {
    const ret = wasm.isVoidOfCourseMoon(jd_ut);
    return ret !== 0;
}

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
 * @param {number} jd_ut
 * @param {number} ipl
 * @param {number | null} [iflag]
 * @returns {any}
 */
export function swe_calc_ut(jd_ut, ipl, iflag) {
    const ret = wasm.swe_calc_ut(jd_ut, ipl, isLikeNone(iflag) ? 0x100000001 : (iflag) >> 0);
    return ret;
}

/**
 * Calculate all planets at once
 *
 * # Arguments
 * * `jd_ut` - Julian Day in Universal Time
 * * `iflag` - Calculation flags
 *
 * # Returns
 * Object with all planet positions
 * @param {number} jd_ut
 * @param {number | null} [iflag]
 * @returns {any}
 */
export function swe_calc_ut_all(jd_ut, iflag) {
    const ret = wasm.swe_calc_ut_all(jd_ut, isLikeNone(iflag) ? 0x100000001 : (iflag) >> 0);
    return ret;
}

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
 * @param {number} jd_ut
 * @param {Int32Array} planets
 * @param {number | null} [iflag]
 * @returns {any}
 */
export function swe_calc_ut_batch(jd_ut, planets, iflag) {
    const ptr0 = passArray32ToWasm0(planets, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.swe_calc_ut_batch(jd_ut, ptr0, len0, isLikeNone(iflag) ? 0x100000001 : (iflag) >> 0);
    return ret;
}

/**
 * Get planet name
 * @param {number} ipl
 * @returns {string}
 */
export function swe_get_planet_name(ipl) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.swe_get_planet_name(ipl);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

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
 * @param {number} jd_ut
 * @param {number} lat
 * @param {number} lon
 * @param {string | null} [_hsys]
 * @returns {any}
 */
export function swe_houses(jd_ut, lat, lon, _hsys) {
    var ptr0 = isLikeNone(_hsys) ? 0 : passStringToWasm0(_hsys, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.swe_houses(jd_ut, lat, lon, ptr0, len0);
    return ret;
}

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
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number | null} [gregflag]
 * @returns {number}
 */
export function swe_julday(year, month, day, hour, gregflag) {
    const ret = wasm.swe_julday(year, month, day, hour, isLikeNone(gregflag) ? 0x100000001 : (gregflag) >> 0);
    return ret;
}

/**
 * Convert Julian Day to calendar date
 *
 * # Arguments
 * * `jd` - Julian Day number
 * * `gregflag` - Calendar flag: 1 for Gregorian (default), 0 for Julian
 *
 * # Returns
 * Object with year, month, day, hour
 * @param {number} jd
 * @param {number | null} [gregflag]
 * @returns {any}
 */
export function swe_revjul(jd, gregflag) {
    const ret = wasm.swe_revjul(jd, isLikeNone(gregflag) ? 0x100000001 : (gregflag) >> 0);
    return ret;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25: function(arg0) {
            const v = arg0;
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_47fa6863be6f2f25: function(arg0, arg1) {
            const ret = arg0 in arg1;
            return ret;
        },
        __wbg___wbindgen_is_function_0095a73b8b156f76: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_5ae8e5880f2c1fbd: function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_cd444516edc5b180: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_9e4d92534c42d778: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_9dd77d8cd6671811: function(arg0, arg1) {
            const ret = arg0 == arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_8ff4255516ccad3e: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72fb696202c56729: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_389efe28435a9388: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_done_57b39ecd9addfe81: function(arg0) {
            const ret = arg0.done;
            return ret;
        },
        __wbg_get_9b94d73e6221f75c: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_b3ed3ad4be2bc8ac: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_with_ref_key_1dc361bd10053bfe: function(arg0, arg1) {
            const ret = arg0[arg1];
            return ret;
        },
        __wbg_instanceof_ArrayBuffer_c367199e2fa2aa04: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_9b9075935c74707c: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isArray_d314bb98fcf08331: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_iterator_6ff6560ca1568e55: function() {
            const ret = Symbol.iterator;
            return ret;
        },
        __wbg_length_32ed9a279acd054c: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_35a7bace40f36eac: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_new_361308b2356cecd0: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_dca287b076112a51: function() {
            const ret = new Map();
            return ret;
        },
        __wbg_new_dd2b680c8bf6ae29: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_next_3482f54c49e8af19: function() { return handleError(function (arg0) {
            const ret = arg0.next();
            return ret;
        }, arguments); },
        __wbg_next_418f80d8f5303233: function(arg0) {
            const ret = arg0.next;
            return ret;
        },
        __wbg_prototypesetcall_bdcdcc5842e4d77d: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_set_1eb0999cf5d27fc8: function(arg0, arg1, arg2) {
            const ret = arg0.set(arg1, arg2);
            return ret;
        },
        __wbg_set_3f1d0b984ed272ed: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_set_f43e577aea94465b: function(arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
        },
        __wbg_value_0546255b415e96c1: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./tailored_ephemeris_bg.js": import0,
    };
}

const SEFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_se_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('tailored_ephemeris_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
