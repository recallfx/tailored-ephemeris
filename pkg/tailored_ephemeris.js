/* @ts-self-types="./tailored_ephemeris.d.ts" */

import * as wasm from "./tailored_ephemeris_bg.wasm";
import { __wbg_set_wasm } from "./tailored_ephemeris_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    SE, calculateChart, computeMundaneAspects, computeMundaneAspectsWithOrbs, computeNatalAspectsWithOrbs, computeTransitAspects, computeTransitAspectsWithOrbs, getAllPlanetaryPositions, getMoonPhase, getNatalChart, getPlanetInHouse, getPlanetaryHourRuler, getSignFromLongitude, isVoidOfCourseMoon, swe_calc_ut, swe_calc_ut_all, swe_calc_ut_batch, swe_get_planet_name, swe_houses, swe_julday, swe_revjul
} from "./tailored_ephemeris_bg.js";
