# tailored-ephemeris

Minimal astronomical ephemeris calculations for horoscope generation. Pure Rust, compiles to WASM, no external data files required.

## Features

- **Self-contained**: All calculations use analytical methods (Moshier/VSOP87-derived), no ephemeris files needed
- **WASM-ready**: First-class WebAssembly support via `wasm-bindgen`
- **Accurate**: <1 arcminute accuracy for planets, tested across 1925-2075
- **Minimal**: ~100KB WASM bundle (gzipped ~40KB)

## Supported Calculations

| Category | Details |
|----------|---------|
| **Planets** | Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto |
| **Points** | True North Node |
| **Houses** | Placidus system |
| **Astrology** | Zodiac signs, moon phases, eclipses, aspects, planetary hours, void-of-course Moon |

## Installation

### Rust

```toml
[dependencies]
tailored-ephemeris = "0.1"
```

### WASM/JavaScript

```bash
wasm-pack build --target web --features wasm
```

```javascript
import init, { swe_julday, swe_calc_ut, swe_houses } from './tailored_ephemeris.js';

await init();

const jd = swe_julday(2024, 6, 21, 12.0, 1);
const sun = swe_calc_ut(jd, 0, 256); // SE_SUN with SEFLG_SPEED
console.log(sun.longitude, sun.speedLong);
```

## API

### Low-level (Swiss Ephemeris compatible)

```rust
use tailored_ephemeris::{julian_day, calc_ut, calc_houses, Planet};

// Julian Day
let jd = julian_day(2024, 6, 21, 12.0);

// Planet position
let sun = calc_ut(jd, Planet::Sun, true)?; // true = include speed
println!("Sun: {}° (speed: {}°/day)", sun.longitude, sun.speed_longitude);

// House cusps (Placidus)
let houses = calc_houses(jd, 47.38, 8.54)?; // latitude, longitude
println!("Ascendant: {}°", houses.ascendant);
```

### High-level Astrology

```rust
use tailored_ephemeris::astrology::*;

// Get complete natal chart
let chart = get_natal_chart(jd, 47.38, 8.54)?;

// Moon phase
let phase = calculate_moon_phase(jd)?;

// Eclipse detection
let eclipse = get_eclipse_type(jd)?; // Some(Solar) / Some(Lunar) / None

// Planetary positions with zodiac signs
let positions = get_all_planetary_positions(jd)?;
for p in positions {
    println!("{}: {}° {} (retrograde: {})",
        p.planet_key, p.sign_degree, p.sign_key, p.is_retrograde);
}

// Aspects between charts
let aspects = compute_aspects(&natal_positions, &transit_positions);

// Void-of-course Moon
let voc = is_void_of_course_moon(jd)?;

// Planetary hour ruler
let ruler = get_planetary_hour_ruler(2024, 6, 21, 14);
```

### WASM API

When compiled with `--features wasm`:

```typescript
// Low-level
swe_julday(year, month, day, hour, gregflag): number
swe_calc_ut(jd, planet, flags): { longitude, latitude, distance, speedLong, speedLat, speedDist }
swe_houses(jd, lat, lon, hsys): { cusps: number[], ascmc: number[] }

// High-level
getAllPlanetaryPositions(jd): PlanetPosition[]
getNatalChart(jd, lat, lon): NatalChart
getMoonPhase(jd): string
getEclipseType(jd): string
isEclipse(jd): boolean
computeTransitAspects(jd, natalPositions): Aspect[]
computeMundaneAspects(jd): Aspect[]
isVoidOfCourseMoon(jd): boolean
getPlanetaryHourRuler(year, month, day, hour): string
```

## Accuracy

### Planetary Positions

Tested against Swiss Ephemeris across 150 years (1925-2075):

| Body | Max Error |
|------|-----------|
| Sun | <1 arcmin |
| Moon | <2 arcmin |
| Mercury-Mars | <1 arcmin |
| Jupiter-Neptune | <5 arcmin |
| Pluto | <16 arcmin |

### House Cusps (Placidus)

Validated against Swiss Ephemeris (pyswisseph v2.10.03) across 4 geographic locations (London, New York, Sydney, Tokyo) spanning 1975-2000:

| Component | Max Error |
|-----------|-----------|
| Ascendant | <0.01° |
| MC | <0.01° |
| Vertex | <0.01° |
| All 12 cusps | <0.01° |

Sufficient for all astrological applications.

## License

GPL-3.0 - see [LICENSE](LICENSE)

## Acknowledgments

Algorithms derived from:
- Moshier's analytical ephemeris methods
- VSOP87 planetary theory
- ELP2000 lunar theory
