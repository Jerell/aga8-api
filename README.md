# aga8-api

A web API for producing thermodynamic data using the [AGA8 crate](https://crates.io/crates/aga8).

This API generates thermodynamic data in the tab file format used by Multiflash.  
The output includes all the metadata like `BUBBLEPRESSURES`, `BUBBLETEMPERATURES`, `CRITICALPRESSURE`, `CRITICALTEMPERATURE`, etc.

## API Endpoints

### POST `/generate-tab-file`

Generates thermodynamic data and returns it as a tab file in Multiflash format.

**Request Body:**

```json
{
  "composition": {
    "components": ["CO2", "H2"],
    "mole_fractions": [0.99, 0.01]
  },
  "pressure_range": {
    "min": 100000.0,
    "max": 8500000.0,
    "points": 100
  },
  "enthalpy_range": {
    "min": -100000.0,
    "max": 100000.0,
    "points": 100
  },
  "equation_of_state": "gerg2008"
}
```

This request had a file generation time of 2.740s on my M1 Macbook Air.

**Parameters:**

- `equation_of_state` (optional, default: `"gerg2008"`): Choose between `"gerg2008"` (GERG-2008 equation of state) or `"aga8detail"` (AGA8 DETAIL equation of state)

**Note:**

- **Pressure range:** Must be within AGA8 valid range: **100,000 Pa (0.1 MPa) to 275,000,000 Pa (275 MPa)**
  - For optimal accuracy (0.1% uncertainty): Up to **12,000,000 Pa (12 MPa)**
- **Temperature range:** Automatically constrained to **-29°C to 200°C** (practical range)
  - Theoretical AGA8 range: **-130°C to 200°C**, but minimum working temperature is pressure-dependent
  - At 0.1 MPa (minimum pressure): minimum is **-29°C**
  - At higher pressures, lower temperatures may work (e.g., -60°C at 1 MPa)
  - For optimal accuracy (0.1% uncertainty): **-8°C to 62°C**
- **Important:** AGA8 is not recommended for use near the critical point or in the liquid phase, and is valid only for lean natural gas mixtures.

**Response:** Plain text tab file

### POST `/generate-data`

Generates thermodynamic data and returns it as JSON.

**Request Body:** Same as `/generate-tab-file`

**Response:** JSON object with thermodynamic data

### GET `/health`

Health check endpoint.

## Running the API

```bash
cargo run
```

The API will start on `http://localhost:3000`
API documentation (Swagger UI) is available at `http://localhost:3000/docs`

## Measuring Request Time

### Using HTTPie

HTTPie can show timing information. Use the `--verbose` flag to see detailed timing:

```bash
http --verbose POST http://localhost:3000/generate-tab-file < request.json
```

Or use the `time` command to measure total request time:

```bash
time http POST http://localhost:3000/generate-tab-file < request.json
```

### Using curl

```bash
time curl -X POST http://localhost:3000/generate-tab-file \
  -H "Content-Type: application/json" \
  -d @request.json \
  -o output.tab
```

### Timing in API Response

The `/generate-data` endpoint includes `generation_time_seconds` in the JSON response. The server also logs generation time to stderr for both endpoints.

## Running Tests

```bash
cargo test
```

## Project Structure

- `src/models.rs` - Data models for requests and thermodynamic data
- `src/aga8_calc.rs` - Integration layer with aga8 crate for calculations
- `src/tab_file.rs` - Tab file formatter for Multiflash format
- `src/service.rs` - Service layer orchestrating data generation
- `src/api.rs` - Poem web API endpoints
- `src/main.rs` - Application entry point
- `tests/` - Comprehensive test suite

## Properties Available from aga8 Crate (GERG-2008)

This API uses the `aga8` Rust crate which provides thermodynamic properties through GERG-2008 (and optionally AGA8 DETAIL) equation of state implementations.

### GERG-2008 Equation of State (Primary)

The aga8 crate's GERG-2008 implementation provides the following properties directly (all accessed from the `Gerg2008` struct after calling `density()` and `properties()`):

- **Density** (d) - mol/l (converted to kg/m³ in our API)
- **Compressibility factor** (z)
- **Internal energy** (u) - J/mol (available from aga8 but not currently used in our output)
- **Enthalpy** (h) - J/mol
- **Entropy** (s) - J/(mol·K)
- **Isobaric heat capacity** (cp) - J/(mol·K)
- **Isochoric heat capacity** (cv) - J/(mol·K)
- **Pressure derivatives** (dp_dd, dp_dt) - kPa/(mol/l) and kPa/K (used to calculate density derivatives)
- **Second pressure derivatives** (d2p_dd2, d2p_dtd) - available from aga8 but not currently used
- **Speed of sound** (w) - m/s
- **Gibbs energy** (g) - J/mol
- **Joule-Thomson coefficient** (jt) - K/kPa
- **Isentropic exponent** (kappa)

### AGA8 DETAIL Equation of State

Provides the same properties as GERG-2008.

### Properties Not Available from aga8 (GERG-2008)

The following properties are **not** provided by the aga8 crate's GERG-2008 implementation:

- **Viscosity** (gas and liquid) - set to 0.0 in output
- **Thermal conductivity** (gas and liquid) - set to 0.0 in output
- **Surface tension** - set to 0.0 in output
- **Phase boundaries** (bubble/dew points) - set to -999 (NaN) in output
- **Critical point** (critical pressure and temperature) - set to -999 (NaN) in output

The aga8 crate's GERG-2008 implementation does not provide:

- **Phase boundary calculations**: The documentation states "No checks are made to determine the phase boundary". Phase boundaries would require two-phase flash calculations which GERG-2008 does not provide.
- **Critical point calculations**: While GERG-2008 has a private `pseudocriticalpoint()` method (used internally for mixing rules), it does not calculate the actual critical point of mixtures, which would require solving the criticality conditions: (dp/dd)\_T = 0 and (d²p/dd²)\_T = 0.

These properties would require additional correlations or separate property packages to calculate.

### Generated tab file format

```
PVTTABLE LABEL = GERG2008_CO2_,_H2 ,  PHASE = TWO, \
!
! Multiflash Version 7.4.08     May 2023
!
! EOS:  GERG-2008                                                                          !
!
COMPONENTS = ( CO2 , H2 ), \
MOLES = ( 0.990000000 , 0.010000000 ), \
MOLWEIGHT = ( 0.044009800 , 0.002015880 ), \
STDPRESSURE = 101325 PA , \
STDTEMPERATURE = 15.55556 C, \
GOR = -999 SM3/SM3, \
GLR = -999 SM3/SM3, \
WC = -999 , \
STDGASDENSITY = 1.85037281 KG/M3, \
STDOILDENSITY = -999 KG/M3, \
STDWATDENSITY = -999 KG/M3, \
MESHTYPE = STANDARD, \
PRESSURE = ( 100000.00 , 107575.76 , 115151.52 , 122727.27 , 130303.03 , \
137878.79 , 145454.55 , 153030.30 , 160606.06 , 168181.82 , 175757.58 , \
183333.33 , 190909.09 , 198484.85 , 206060.61 , 213636.36 , 221212.12 , \
228787.88 , 236363.64 , 243939.39 , 251515.15 , 259090.91 , 266666.67 , \
274242.42 , 281818.18 , 289393.94 , 296969.70 , 304545.45 , 312121.21 , \
319696.97 , 327272.73 , 334848.48 , 342424.24 , 350000.00 , 357575.76 , \
365151.52 , 372727.27 , 380303.03 , 387878.79 , 395454.55 , 403030.30 , \
410606.06 , 418181.82 , 425757.58 , 433333.33 , 440909.09 , 448484.85 , \
456060.61 , 463636.36 , 471212.12 , 478787.88 , 486363.64 , 493939.39 , \
501515.15 , 509090.91 , 516666.67 , 524242.42 , 531818.18 , 539393.94 , \
546969.70 , 554545.45 , 562121.21 , 569696.97 , 577272.73 , 584848.48 , \
592424.24 , 600000.00 , 607575.76 , 615151.52 , 622727.27 , 630303.03 , \
637878.79 , 645454.55 , 653030.30 , 660606.06 , 668181.82 , 675757.58 , \
683333.33 , 690909.09 , 698484.85 , 706060.61 , 713636.36 , 721212.12 , \
728787.88 , 736363.64 , 743939.39 , 751515.15 , 759090.91 , 766666.67 , \
774242.42 , 781818.18 , 789393.94 , 796969.70 , 804545.45 , 812121.21 , \
819696.97 , 827272.73 , 834848.48 , 842424.24 , 850000.00 ) PA, \
TEMPERATURE = ( -29.00 , -26.69 , -24.37 , -22.06 , -19.75 , -17.43 , -15.12 , \
-12.81 , -10.49 , -8.18 , -5.87 , -3.56 , -1.24 , 1.07 , 3.38 , 5.70 , 8.01 , \
10.32 , 12.64 , 14.95 , 17.26 , 19.58 , 21.89 , 24.20 , 26.52 , 28.83 , 31.14 , \
33.45 , 35.77 , 38.08 , 40.39 , 42.71 , 45.02 , 47.33 , 49.65 , 51.96 , 54.27 , \
56.59 , 58.90 , 61.21 , 63.53 , 65.84 , 68.15 , 70.46 , 72.78 , 75.09 , 77.40 , \
79.72 , 82.03 , 84.34 , 86.66 , 88.97 , 91.28 , 93.60 , 95.91 , 98.22 , \
100.54 , 102.85 , 105.16 , 107.47 , 109.79 , 112.10 , 114.41 , 116.73 , \
119.04 , 121.35 , 123.67 , 125.98 , 128.29 , 130.61 , 132.92 , 135.23 , \
137.55 , 139.86 , 142.17 , 144.48 , 146.80 , 149.11 , 151.42 , 153.74 , \
156.05 , 158.36 , 160.68 , 162.99 , 165.30 , 167.62 , 169.93 , 172.24 , \
174.56 , 176.87 , 179.18 , 181.49 , 183.81 , 186.12 , 188.43 , 190.75 , \
193.06 , 195.37 , 197.69 , 200.00 ) C, \
BUBBLEPRESSURES = ( 109687795.33 , 105459090.77 , 101230386.20 , 97001681.63 , \
92772977.06 , 88544272.50 , 84315567.93 , 80086863.36 , 75858158.79 , \
71629454.23 , 67400749.66 , 63172045.09 , 58943340.52 , 54714635.95 , \
50485931.39 , 46257226.82 , 42028522.25 , 37799817.68 , 33571113.12 , \
29342408.55 , 25113703.98 , 20884999.41 , 16656294.85 , 12427590.28 , \
8198885.71 , 3970181.14 , -258523.42 , -4487227.99 , -8715932.56 , \
-12944637.13 , -17173341.69 , -21402046.26 , -25630750.83 , -29859455.40 , \
-34088159.96 , -38316864.53 , -42545569.10 , -46774273.67 , -51002978.23 , \
-55231682.80 , -59460387.37 , -63689091.94 , -67917796.50 , -72146501.07 , \
-76375205.64 , -80603910.21 , -84832614.77 , -89061319.34 , -93290023.91 , \
-97518728.48 , -101747433.04 , -105976137.61 , -110204842.18 , -114433546.75 , \
-118662251.32 , -122890955.88 , -127119660.45 , -131348365.02 , -135577069.59 , \
-139805774.15 , -144034478.72 , -148263183.29 , -152491887.86 , -156720592.42 , \
-160949296.99 , -165178001.56 , -169406706.13 , -173635410.69 , -177864115.26 , \
-182092819.83 , -186321524.40 , -190550228.96 , -194778933.53 , -199007638.10 , \
-203236342.67 , -207465047.23 , -211693751.80 , -215922456.37 , -220151160.94 , \
-224379865.50 , -228608570.07 , -232837274.64 , -237065979.21 , -241294683.77 , \
-245523388.34 , -249752092.91 , -253980797.48 , -258209502.04 , -262438206.61 , \
-266666911.18 , -270895615.75 , -275124320.32 , -279353024.88 , -283581729.45 , \
-287810434.02 , -292039138.59 , -296267843.15 , -300496547.72 , -304725252.29 , \
-308953956.86 ) PA, \
BUBBLETEMPERATURES = ( 30.95 , 30.94 , 30.94 , 30.93 , 30.93 , 30.92 , 30.92 , \
30.92 , 30.91 , 30.91 , 30.90 , 30.90 , 30.90 , 30.89 , 30.89 , 30.88 , 30.88 , \
30.87 , 30.87 , 30.87 , 30.86 , 30.86 , 30.85 , 30.85 , 30.85 , 30.84 , 30.84 , \
30.83 , 30.83 , 30.83 , 30.82 , 30.82 , 30.81 , 30.81 , 30.80 , 30.80 , 30.80 , \
30.79 , 30.79 , 30.78 , 30.78 , 30.78 , 30.77 , 30.77 , 30.76 , 30.76 , 30.75 , \
30.75 , 30.75 , 30.74 , 30.74 , 30.73 , 30.73 , 30.73 , 30.72 , 30.72 , 30.71 , \
30.71 , 30.70 , 30.70 , 30.70 , 30.69 , 30.69 , 30.68 , 30.68 , 30.68 , 30.67 , \
30.67 , 30.66 , 30.66 , 30.66 , 30.65 , 30.65 , 30.64 , 30.64 , 30.63 , 30.63 , \
30.63 , 30.62 , 30.62 , 30.61 , 30.61 , 30.61 , 30.60 , 30.60 , 30.59 , 30.59 , \
30.58 , 30.58 , 30.58 , 30.57 , 30.57 , 30.56 , 30.56 , 30.56 , 30.55 , 30.55 , \
30.54 , 30.54 , 30.54 ) C, \
DEWPRESSURES = ( 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 , \
100000.00 , 100000.00 , 100000.00 , 100000.00 , 100000.00 ) PA, \
DEWTEMPERATURES = ( -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 , \
-123.15 , -123.15 , -123.15 , -123.15 , -123.15 , -123.15 ) C, \
CRITICALPRESSURE = ( 6179493.04 ) PA, \
CRITICALTEMPERATURE = ( 176.4101392 ) C, \
COLUMNS = ( PT, TM, ROG, ROHL, DROGDP,  DROHLDP, DROGDT, DROHLDT, RS, VISG, VISHL, CPG, CPHL,   HG, HHL, TCG, TCHL, SIGGHL, SEG, SEHL)
PVTTABLE POINT = ( 100000.00 , -29.000000000 , 1232.75417659 , 1232.75418 , 0.00000000 , 0.00000000 , -0.00001111 , -0.0000111 , 1.00000000 , 0.00000000 , 0.000000000 , 1690.726807 , 1690.72681 ,   -14631.334 , -14631.33 , 0.000000000 , 0.000000000 , 0.000000000 , -95.317901 , -95.3179 )
PVTTABLE POINT = ( 100000.00 , -26.686868687 , 1227.67298765 , 1227.67299 , 0.00000000 , 0.00000000 , -0.00001093 , -0.0000109 , 1.00000000 , 0.00000000 , 0.000000000 , 1687.054979 , 1687.05498 ,   -14461.045 , -14461.05 , 0.000000000 , 0.000000000 , 0.000000000 , -94.623707 , -94.6237 )
PVTTABLE POINT = ( 100000.00 , -24.373737374 , 1222.60479378 , 1222.60479 , 0.00000000 , 0.00000000 , -0.00001075 , -0.0000108 , 1.00000000 , 0.00000000 , 0.000000000 , 1683.458867 , 1683.45887 ,   -14291.123 , -14291.12 , 0.000000000 , 0.000000000 , 0.000000000 , -93.937478 , -93.9375 )
PVTTABLE POINT = ( 100000.00 , -22.060606061 , 1217.54952837 , 1217.54953 , 0.00000000 , 0.00000000 , -0.00001058 , -0.0000106 , 1.00000000 , 0.00000000 , 0.000000000 , 1679.936633 , 1679.93663 ,   -14121.559 , -14121.56 , 0.000000000 , 0.000000000 , 0.000000000 , -93.259037 , -93.2590 )
PVTTABLE POINT = ( 100000.00 , -19.747474747 , 1212.50712904 , 1212.50713 , 0.00000000 , 0.00000000 , -0.00001042 , -0.0000104 , 1.00000000 , 0.00000000 , 0.000000000 , 1676.486459 , 1676.48646 ,   -13952.347 , -13952.35 , 0.000000000 , 0.000000000 , 0.000000000 , -92.588210 , -92.5882 )
PVTTABLE POINT = ( 100000.00 , -17.434343434 , 1207.47753789 , 1207.47754 , 0.00000000 , 0.00000000 , -0.00001025 , -0.0000103 , 1.00000000 , 0.00000000 , 0.000000000 , 1673.106548 , 1673.10655 ,   -13783.480 , -13783.48 , 0.000000000 , 0.000000000 , 0.000000000 , -91.924831 , -91.9248 )
PVTTABLE POINT = ( 100000.00 , -15.121212121 , 1202.46070165 , 1202.46070 , 0.00000000 , 0.00000000 , -0.00001009 , -0.0000101 , 1.00000000 , 0.00000000 , 0.000000000 , 1669.795123 , 1669.79512 ,   -13614.950 , -13614.95 , 0.000000000 , 0.000000000 , 0.000000000 , -91.268740 , -91.2687 )
```
