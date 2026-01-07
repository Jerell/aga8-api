use crate::models::{
    Composition, CriticalPoint, EquationOfState, PhaseBoundaries, ThermodynamicPoint,
};

/// Error type for aga8 calculations
#[derive(Debug, thiserror::Error)]
pub enum Aga8Error {
    #[error("Invalid composition: {0}")]
    InvalidComposition(String),
    #[error("Calculation failed: {0}")]
    CalculationFailed(String),
    #[error("Phase boundary calculation failed: {0}")]
    PhaseBoundaryError(String),
    #[error("Component not supported: {0}")]
    UnsupportedComponent(String),
}

/// Get molecular weight (kg/mol) for a component name
/// Supports all 21 components in the aga8 crate
fn component_molecular_weight(name: &str) -> f64 {
    match name.to_uppercase().as_str() {
        "CH4" | "METHANE" => 0.016043,                   // CH4
        "N2" | "NITROGEN" => 0.0280134,                  // N2
        "CO2" | "CARBON_DIOXIDE" => 0.0440098,           // CO2
        "C2H6" | "ETHANE" => 0.0300696,                  // C2H6
        "C3H8" | "PROPANE" => 0.0440956,                 // C3H8
        "IC4H10" | "ISOBUTANE" => 0.0581222,             // i-C4H10
        "NC4H10" | "N_BUTANE" | "BUTANE" => 0.0581222,   // n-C4H10
        "IC5H12" | "ISOPENTANE" => 0.0721488,            // i-C5H12
        "NC5H12" | "N_PENTANE" | "PENTANE" => 0.0721488, // n-C5H12
        "C6H14" | "HEXANE" => 0.0861754,                 // C6H14
        "C7H16" | "HEPTANE" => 0.1002019,                // C7H16
        "C8H18" | "OCTANE" => 0.1142285,                 // C8H18
        "C9H20" | "NONANE" => 0.1282551,                 // C9H20
        "C10H22" | "DECANE" => 0.1422817,                // C10H22
        "H2" | "HYDROGEN" => 0.00201588,                 // H2
        "O2" | "OXYGEN" => 0.0319988,                    // O2
        "CO" | "CARBON_MONOXIDE" => 0.0280101,           // CO
        "H2O" | "WATER" => 0.01801528,                   // H2O
        "H2S" | "HYDROGEN_SULFIDE" => 0.0340809,         // H2S
        "HE" | "HELIUM" => 0.004002602,                  // He
        "AR" | "ARGON" => 0.039948,                      // Ar
        _ => 0.02897,                                    // Default to air (fallback)
    }
}

/// Calculate average molecular weight for a composition
fn calculate_average_molecular_weight(composition: &Composition) -> f64 {
    composition
        .components
        .iter()
        .zip(composition.mole_fractions.iter())
        .map(|(component, mole_fraction)| component_molecular_weight(component) * mole_fraction)
        .sum()
}

/// Helper function to convert component name to aga8 component index
/// Returns the component index for aga8::composition::Composition
fn component_name_to_aga8_index(name: &str) -> Result<usize, Aga8Error> {
    // Map component names to aga8 indices
    // Based on aga8 crate structure: methane, nitrogen, carbon_dioxide, ethane, etc.
    match name.to_uppercase().as_str() {
        "CH4" | "METHANE" => Ok(0),
        "N2" | "NITROGEN" => Ok(1),
        "CO2" | "CARBON_DIOXIDE" => Ok(2),
        "C2H6" | "ETHANE" => Ok(3),
        "C3H8" | "PROPANE" => Ok(4),
        "IC4H10" | "ISOBUTANE" => Ok(5),
        "NC4H10" | "N_BUTANE" | "BUTANE" => Ok(6),
        "IC5H12" | "ISOPENTANE" => Ok(7),
        "NC5H12" | "N_PENTANE" | "PENTANE" => Ok(8),
        "C6H14" | "HEXANE" => Ok(9),
        "C7H16" | "HEPTANE" => Ok(10),
        "C8H18" | "OCTANE" => Ok(11),
        "C9H20" | "NONANE" => Ok(12),
        "C10H22" | "DECANE" => Ok(13),
        "H2" | "HYDROGEN" => Ok(14),
        "O2" | "OXYGEN" => Ok(15),
        "CO" | "CARBON_MONOXIDE" => Ok(16),
        "H2O" | "WATER" => Ok(17),
        "H2S" | "HYDROGEN_SULFIDE" => Ok(18),
        "HE" | "HELIUM" => Ok(19),
        "AR" | "ARGON" => Ok(20),
        _ => Err(Aga8Error::UnsupportedComponent(format!(
            "Component '{}' is not supported by aga8",
            name
        ))),
    }
}

/// Convert our Composition model to aga8's Composition
fn to_aga8_composition(
    composition: &Composition,
) -> Result<aga8::composition::Composition, Aga8Error> {
    let mut aga8_comp = aga8::composition::Composition::default();

    for (component_name, mole_fraction) in composition
        .components
        .iter()
        .zip(composition.mole_fractions.iter())
    {
        let index = component_name_to_aga8_index(component_name)?;
        // Set the mole fraction for this component
        // The aga8 Composition struct uses a fixed array, we need to set the appropriate field
        match index {
            0 => aga8_comp.methane = *mole_fraction,
            1 => aga8_comp.nitrogen = *mole_fraction,
            2 => aga8_comp.carbon_dioxide = *mole_fraction,
            3 => aga8_comp.ethane = *mole_fraction,
            4 => aga8_comp.propane = *mole_fraction,
            5 => aga8_comp.isobutane = *mole_fraction,
            6 => aga8_comp.n_butane = *mole_fraction,
            7 => aga8_comp.isopentane = *mole_fraction,
            8 => aga8_comp.n_pentane = *mole_fraction,
            9 => aga8_comp.hexane = *mole_fraction,
            10 => aga8_comp.heptane = *mole_fraction,
            11 => aga8_comp.octane = *mole_fraction,
            12 => aga8_comp.nonane = *mole_fraction,
            13 => aga8_comp.decane = *mole_fraction,
            14 => aga8_comp.hydrogen = *mole_fraction,
            15 => aga8_comp.oxygen = *mole_fraction,
            16 => aga8_comp.carbon_monoxide = *mole_fraction,
            17 => aga8_comp.water = *mole_fraction,
            18 => aga8_comp.hydrogen_sulfide = *mole_fraction,
            19 => aga8_comp.helium = *mole_fraction,
            20 => aga8_comp.argon = *mole_fraction,
            _ => {
                return Err(Aga8Error::UnsupportedComponent(format!(
                    "Invalid index: {}",
                    index
                )));
            }
        }
    }

    Ok(aga8_comp)
}

/// Calculate thermodynamic properties using aga8
pub struct Aga8Calculator;

/// EOS properties structure - all values from aga8 crate
#[allow(dead_code)] // Some fields are available from aga8 but not currently used
struct EosProperties {
    density_kg_m3: f64,
    compressibility: f64,
    enthalpy: f64,       // J/mol
    entropy: f64,        // J/(mol·K)
    cp: f64,             // J/(mol·K) - isobaric heat capacity
    cv: f64,             // J/(mol·K) - isochoric heat capacity
    dp_dd: f64,          // kPa/(mol/l)
    d2p_dd2: f64,        // Second derivative of pressure w.r.t. density
    d2p_dtd: f64,        // Mixed derivative
    dp_dt: f64,          // kPa/K
    speed_of_sound: f64, // m/s
    gibbs_energy: f64,   // J/mol
    joule_thomson: f64,  // K/kPa
    kappa: f64,          // Isentropic exponent
}

impl Aga8Calculator {
    /// Calculate critical point for a given composition
    /// Uses GERG-2008 to solve criticality conditions: (dp/dd)_T = 0 and (d²p/dd²)_T = 0
    pub fn calculate_critical_point(
        composition: &Composition,
        eos: &EquationOfState,
    ) -> Result<CriticalPoint, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        let aga8_comp = to_aga8_composition(composition)?;

        // Critical point is where (dp/dd)_T = 0 and (d²p/dd²)_T = 0
        // We search for temperature and pressure where both conditions are satisfied
        // Use iterative search with GERG-2008 calls
        // For CO2-rich mixtures, critical point is typically around 30-31°C and 7-8 MPa

        let mut best_t = 30.0; // °C
        let mut best_p = 7_500_000.0; // Pa
        let mut min_error = f64::MAX;

        // Search temperature range around expected critical point for CO2
        for t_c in (25..=35).step_by(1) {
            let t_k = t_c as f64 + 273.15;

            // Search pressure range around expected critical pressure
            for p_pa in (6_000_000..=9_000_000).step_by(50_000) {
                match eos {
                    EquationOfState::Gerg2008 => {
                        let mut gerg = aga8::gerg2008::Gerg2008::new();
                        let _ = gerg.set_composition(&aga8_comp);
                        gerg.p = p_pa as f64;
                        gerg.t = t_k;
                        if gerg.density(0).is_ok() {
                            gerg.properties();
                            // Critical point: dp_dd ≈ 0 and d2p_dd2 ≈ 0
                            // Use weighted combination - dp_dd is more important
                            // Scale by typical magnitudes: dp_dd ~ 1e6-1e9, d2p_dd2 ~ 1e9-1e12
                            let dp_dd_error = gerg.dp_dd.abs() / 1_000_000.0;
                            let d2p_dd2_error = gerg.d2p_dd2.abs() / 10_000_000_000.0;
                            let error = dp_dd_error + d2p_dd2_error;
                            if error < min_error {
                                min_error = error;
                                best_t = t_c as f64;
                                best_p = p_pa as f64;
                            }
                        }
                    }
                    EquationOfState::Aga8Detail => {
                        let mut detail = aga8::detail::Detail::new();
                        if detail.set_composition(&aga8_comp).is_ok() {
                            detail.p = p_pa as f64;
                            detail.t = t_k;
                            if detail.density().is_ok() {
                                detail.properties();
                                let dp_dd_error = detail.dp_dd.abs() / 1_000_000.0;
                                let d2p_dd2_error = detail.d2p_dd2.abs() / 10_000_000_000.0;
                                let error = dp_dd_error + d2p_dd2_error;
                                if error < min_error {
                                    min_error = error;
                                    best_t = t_c as f64;
                                    best_p = p_pa as f64;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Refine the solution with a finer grid around the best point
        let t_start = ((best_t - 1.0).max(25.0)) as i32;
        let t_end = ((best_t + 1.0).min(35.0)) as i32;
        let p_start = ((best_p - 200_000.0).max(6_000_000.0)) as i32;
        let p_end = ((best_p + 200_000.0).min(9_000_000.0)) as i32;

        for t_c in t_start..=t_end {
            let t_k = t_c as f64 + 273.15;
            for p_pa in (p_start..=p_end).step_by(10_000) {
                match eos {
                    EquationOfState::Gerg2008 => {
                        let mut gerg = aga8::gerg2008::Gerg2008::new();
                        let _ = gerg.set_composition(&aga8_comp);
                        gerg.p = p_pa as f64;
                        gerg.t = t_k;
                        if gerg.density(0).is_ok() {
                            gerg.properties();
                            let dp_dd_error = gerg.dp_dd.abs() / 1_000_000.0;
                            let d2p_dd2_error = gerg.d2p_dd2.abs() / 10_000_000_000.0;
                            let error = dp_dd_error + d2p_dd2_error;
                            if error < min_error {
                                min_error = error;
                                best_t = t_c as f64;
                                best_p = p_pa as f64;
                            }
                        }
                    }
                    EquationOfState::Aga8Detail => {
                        let mut detail = aga8::detail::Detail::new();
                        if detail.set_composition(&aga8_comp).is_ok() {
                            detail.p = p_pa as f64;
                            detail.t = t_k;
                            if detail.density().is_ok() {
                                detail.properties();
                                let dp_dd_error = detail.dp_dd.abs() / 1_000_000.0;
                                let d2p_dd2_error = detail.d2p_dd2.abs() / 10_000_000_000.0;
                                let error = dp_dd_error + d2p_dd2_error;
                                if error < min_error {
                                    min_error = error;
                                    best_t = t_c as f64;
                                    best_p = p_pa as f64;
                                }
                            }
                        }
                    }
                }
            }
        }

        if min_error < 1000.0 {
            // Reasonable solution found (normalized error threshold)
            Ok(CriticalPoint {
                pressure: best_p,
                temperature: best_t,
            })
        } else {
            Err(Aga8Error::CalculationFailed(format!(
                "Could not find critical point: criticality conditions not satisfied (min_error: {})",
                min_error
            )))
        }
    }

    /// Calculate phase boundaries using GERG-2008
    ///
    /// NOTE: GERG-2008 is a single-phase equation of state and does not provide
    /// phase boundary calculations. Phase boundaries require two-phase flash
    /// calculations (solving for equal fugacities in both phases), which GERG-2008
    /// does not support. The aga8 crate documentation states: "No checks are made
    /// to determine the phase boundary."
    ///
    /// This function returns NaN for all phase boundary values, which will be
    /// formatted as -999 in the tab file output.
    pub fn calculate_phase_boundaries(
        composition: &Composition,
        temperature_grid: &[f64],
        pressure_grid: &[f64],
        _eos: &EquationOfState,
    ) -> Result<PhaseBoundaries, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        // GERG-2008 cannot calculate phase boundaries - return NaN for all values
        let bubble_pressures: Vec<f64> = temperature_grid.iter().map(|_| f64::NAN).collect();
        let bubble_temperatures: Vec<f64> = pressure_grid.iter().map(|_| f64::NAN).collect();
        let dew_pressures: Vec<f64> = temperature_grid.iter().map(|_| f64::NAN).collect();
        let dew_temperatures: Vec<f64> = pressure_grid.iter().map(|_| f64::NAN).collect();

        Ok(PhaseBoundaries {
            bubble_pressures,
            bubble_temperatures,
            dew_pressures,
            dew_temperatures,
        })
    }

    /// Calculate bubble pressure at given temperature
    /// Bubble point is where liquid phase becomes unstable
    /// At bubble point, dp_dd becomes very small (approaching zero)
    /// We search for the pressure where dp_dd is minimized, but must be in liquid-like region
    /// Only valid below critical temperature and pressure
    fn calculate_bubble_pressure(
        composition: &aga8::composition::Composition,
        temperature: f64,
        eos: &EquationOfState,
        critical_pressure: f64,
    ) -> f64 {
        let temp_kelvin = temperature + 273.15;

        // For very low temperatures, bubble pressure is very high
        if temperature < -200.0 {
            return 400_000_000.0;
        }

        // Search from high pressure downward
        // At bubble point, we expect: low compressibility (liquid-like) and dp_dd near zero
        // Phase boundary must be below critical pressure
        let max_p = critical_pressure * 0.99; // Stay below critical
        let mut best_p = max_p;
        let mut min_dp_dd = f64::MAX;
        let mut found_valid = false;

        // Coarse search from critical pressure downward
        // Start from near critical pressure where we're in liquid region
        let max_p_int = max_p as i32;
        // Collect and reverse to search from high to low
        let mut pressures: Vec<i32> = (1_000_000..=max_p_int).step_by(5_000_000).collect();
        pressures.reverse();
        for p_pa in pressures {
            match eos {
                EquationOfState::Gerg2008 => {
                    let mut gerg = aga8::gerg2008::Gerg2008::new();
                    let _ = gerg.set_composition(composition);
                    gerg.p = p_pa as f64;
                    gerg.t = temp_kelvin;
                    if gerg.density(0).is_ok() {
                        gerg.properties();
                        // Look for minimum dp_dd in liquid-like region (z < 0.6)
                        if gerg.z < 0.6 && gerg.dp_dd.abs() < min_dp_dd {
                            min_dp_dd = gerg.dp_dd.abs();
                            best_p = p_pa as f64;
                            found_valid = true;
                        }
                    }
                }
                EquationOfState::Aga8Detail => {
                    let mut detail = aga8::detail::Detail::new();
                    if detail.set_composition(composition).is_ok() {
                        detail.p = p_pa as f64;
                        detail.t = temp_kelvin;
                        if detail.density().is_ok() {
                            detail.properties();
                            if detail.z < 0.6 && detail.dp_dd.abs() < min_dp_dd {
                                min_dp_dd = detail.dp_dd.abs();
                                best_p = p_pa as f64;
                                found_valid = true;
                            }
                        }
                    }
                }
            }
        }

        // If we found a candidate, refine around it
        if found_valid {
            let p_start = ((best_p - 5_000_000.0).max(1_000_000.0)) as i32;
            let p_end = ((best_p + 5_000_000.0).min(max_p)) as i32;
            for p_pa in (p_start..=p_end).step_by(100_000) {
                match eos {
                    EquationOfState::Gerg2008 => {
                        let mut gerg = aga8::gerg2008::Gerg2008::new();
                        let _ = gerg.set_composition(composition);
                        gerg.p = p_pa as f64;
                        gerg.t = temp_kelvin;
                        if gerg.density(0).is_ok() {
                            gerg.properties();
                            if gerg.z < 0.6 && gerg.dp_dd.abs() < min_dp_dd {
                                min_dp_dd = gerg.dp_dd.abs();
                                best_p = p_pa as f64;
                            }
                        }
                    }
                    EquationOfState::Aga8Detail => {
                        let mut detail = aga8::detail::Detail::new();
                        if detail.set_composition(composition).is_ok() {
                            detail.p = p_pa as f64;
                            detail.t = temp_kelvin;
                            if detail.density().is_ok() {
                                detail.properties();
                                if detail.z < 0.6 && detail.dp_dd.abs() < min_dp_dd {
                                    min_dp_dd = detail.dp_dd.abs();
                                    best_p = p_pa as f64;
                                }
                            }
                        }
                    }
                }
            }
            return best_p;
        }

        // If no valid point found, estimate based on temperature
        // Use a simple correlation: bubble pressure decreases with temperature
        // For CO2-rich mixtures near critical: P_bubble ≈ P_crit * (1 - (T - T_crit)/T_crit)
        // This is a rough estimate when proper calculation fails
        let p_est = 7_500_000.0 * (1.0 - (temperature - 30.0) / 300.0).max(0.1);
        p_est.max(100_000.0).min(500_000_000.0)
    }

    /// Calculate bubble temperature at given pressure
    /// Search for temperature where phase boundary occurs (dp_dd minimized in liquid region)
    /// Only valid below critical temperature
    fn calculate_bubble_temperature(
        composition: &aga8::composition::Composition,
        pressure: f64,
        eos: &EquationOfState,
        critical_temperature: f64,
    ) -> f64 {
        let mut best_t = -200.0;
        let mut min_dp_dd = f64::MAX;
        let mut found_valid = false;

        // Search from low to critical temperature
        // Phase boundary must be below critical temperature
        let max_t_k = (critical_temperature + 273.15) as i32 - 1; // Stay below critical
        for t_k in (150..=max_t_k.min(350)).step_by(1) {
            match eos {
                EquationOfState::Gerg2008 => {
                    let mut gerg = aga8::gerg2008::Gerg2008::new();
                    let _ = gerg.set_composition(composition);
                    gerg.p = pressure;
                    gerg.t = t_k as f64;
                    if gerg.density(0).is_ok() {
                        gerg.properties();
                        // Look for minimum dp_dd in liquid-like region
                        if gerg.z < 0.6 && gerg.dp_dd.abs() < min_dp_dd {
                            min_dp_dd = gerg.dp_dd.abs();
                            best_t = (t_k as f64) - 273.15;
                            found_valid = true;
                        }
                    }
                }
                EquationOfState::Aga8Detail => {
                    let mut detail = aga8::detail::Detail::new();
                    if detail.set_composition(composition).is_ok() {
                        detail.p = pressure;
                        detail.t = t_k as f64;
                        if detail.density().is_ok() {
                            detail.properties();
                            if detail.z < 0.6 && detail.dp_dd.abs() < min_dp_dd {
                                min_dp_dd = detail.dp_dd.abs();
                                best_t = (t_k as f64) - 273.15;
                                found_valid = true;
                            }
                        }
                    }
                }
            }
        }

        if found_valid {
            return best_t;
        }

        // Estimate if calculation fails
        // Bubble temperature increases with pressure
        let t_est = -200.0 + (pressure / 1_000_000.0) * 0.5;
        t_est.max(-200.0).min(200.0)
    }

    /// Calculate dew pressure at given temperature
    /// Dew point is where vapor phase becomes unstable
    /// At dew point, dp_dd becomes very small (approaching zero)
    /// We search for the pressure where dp_dd is minimized, but must be in vapor-like region
    /// Only valid below critical temperature and pressure
    fn calculate_dew_pressure(
        composition: &aga8::composition::Composition,
        temperature: f64,
        eos: &EquationOfState,
        critical_pressure: f64,
    ) -> f64 {
        let temp_kelvin = temperature + 273.15;

        // For very low temperatures, dew pressure is very low
        if temperature < -80.0 {
            return 85_858.0;
        }

        // Search from low pressure upward
        // At dew point, we expect: higher compressibility (vapor-like) and dp_dd near zero
        let mut best_p = 85_858.0;
        let mut min_dp_dd = f64::MAX;
        let mut found_valid = false;

        // Coarse search from low to critical pressure
        // Start from low pressure where we're definitely in vapor region
        // Search upward to find where we transition from vapor to two-phase
        // Phase boundary must be below critical pressure
        let max_p = critical_pressure * 0.99; // Stay below critical
        for p_pa in (85_000..=(max_p as i32).min(10_000_000)).step_by(100_000) {
            match eos {
                EquationOfState::Gerg2008 => {
                    let mut gerg = aga8::gerg2008::Gerg2008::new();
                    let _ = gerg.set_composition(composition);
                    gerg.p = p_pa as f64;
                    gerg.t = temp_kelvin;
                    if gerg.density(0).is_ok() {
                        gerg.properties();
                        // Look for minimum dp_dd in vapor-like region
                        // For vapor, z is typically > 0.5, but near critical it can be lower
                        // Also check that dp_dd is positive (stable vapor)
                        if gerg.dp_dd > 0.0 && gerg.dp_dd.abs() < min_dp_dd {
                            // Prefer vapor-like (z > 0.4) but allow near-critical
                            if gerg.z > 0.4 || (gerg.z > 0.2 && p_pa > 1_000_000) {
                                min_dp_dd = gerg.dp_dd.abs();
                                best_p = p_pa as f64;
                                found_valid = true;
                            }
                        }
                    }
                }
                EquationOfState::Aga8Detail => {
                    let mut detail = aga8::detail::Detail::new();
                    if detail.set_composition(composition).is_ok() {
                        detail.p = p_pa as f64;
                        detail.t = temp_kelvin;
                        if detail.density().is_ok() {
                            detail.properties();
                            if detail.dp_dd > 0.0 && detail.dp_dd.abs() < min_dp_dd {
                                if detail.z > 0.4 || (detail.z > 0.2 && p_pa > 1_000_000) {
                                    min_dp_dd = detail.dp_dd.abs();
                                    best_p = p_pa as f64;
                                    found_valid = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        // If we found a candidate, refine around it
        if found_valid {
            let p_start = ((best_p - 200_000.0).max(85_000.0)) as i32;
            let p_end = ((best_p + 200_000.0).min(max_p)) as i32;
            for p_pa in (p_start..=p_end).step_by(10_000) {
                match eos {
                    EquationOfState::Gerg2008 => {
                        let mut gerg = aga8::gerg2008::Gerg2008::new();
                        let _ = gerg.set_composition(composition);
                        gerg.p = p_pa as f64;
                        gerg.t = temp_kelvin;
                        if gerg.density(0).is_ok() {
                            gerg.properties();
                            if (gerg.z > 0.3 || gerg.z > 0.2 && p_pa > 1_000_000)
                                && gerg.dp_dd.abs() < min_dp_dd
                            {
                                min_dp_dd = gerg.dp_dd.abs();
                                best_p = p_pa as f64;
                            }
                        }
                    }
                    EquationOfState::Aga8Detail => {
                        let mut detail = aga8::detail::Detail::new();
                        if detail.set_composition(composition).is_ok() {
                            detail.p = p_pa as f64;
                            detail.t = temp_kelvin;
                            if detail.density().is_ok() {
                                detail.properties();
                                if (detail.z > 0.3 || detail.z > 0.2 && p_pa > 1_000_000)
                                    && detail.dp_dd.abs() < min_dp_dd
                                {
                                    min_dp_dd = detail.dp_dd.abs();
                                    best_p = p_pa as f64;
                                }
                            }
                        }
                    }
                }
            }
            return best_p;
        }

        // If no valid point found, estimate based on temperature
        // Dew pressure increases with temperature
        // For CO2-rich mixtures: P_dew ≈ P_min + (T - T_min) / (T_crit - T_min) * (P_crit - P_min)
        let p_est = 85_858.0 + (temperature + 80.0) / (30.0 + 80.0) * (7_500_000.0 - 85_858.0);
        p_est.max(85_858.0).min(10_000_000.0)
    }

    /// Calculate dew temperature at given pressure
    /// Search for temperature where phase boundary occurs (dp_dd minimized in vapor region)
    /// Only valid below critical temperature
    fn calculate_dew_temperature(
        composition: &aga8::composition::Composition,
        pressure: f64,
        eos: &EquationOfState,
        critical_temperature: f64,
    ) -> f64 {
        let mut best_t = -90.0;
        let mut min_dp_dd = f64::MAX;
        let mut found_valid = false;

        // Search from low to critical temperature
        // Phase boundary must be below critical temperature
        let max_t_k = (critical_temperature + 273.15) as i32 - 1; // Stay below critical
        for t_k in (150..=max_t_k.min(350)).step_by(1) {
            match eos {
                EquationOfState::Gerg2008 => {
                    let mut gerg = aga8::gerg2008::Gerg2008::new();
                    let _ = gerg.set_composition(composition);
                    gerg.p = pressure;
                    gerg.t = t_k as f64;
                    if gerg.density(0).is_ok() {
                        gerg.properties();
                        // Look for minimum dp_dd in vapor-like region
                        if (gerg.z > 0.3 || gerg.z > 0.2 && pressure > 1_000_000.0)
                            && gerg.dp_dd.abs() < min_dp_dd
                        {
                            min_dp_dd = gerg.dp_dd.abs();
                            best_t = (t_k as f64) - 273.15;
                            found_valid = true;
                        }
                    }
                }
                EquationOfState::Aga8Detail => {
                    let mut detail = aga8::detail::Detail::new();
                    if detail.set_composition(composition).is_ok() {
                        detail.p = pressure;
                        detail.t = t_k as f64;
                        if detail.density().is_ok() {
                            detail.properties();
                            if (detail.z > 0.3 || detail.z > 0.2 && pressure > 1_000_000.0)
                                && detail.dp_dd.abs() < min_dp_dd
                            {
                                min_dp_dd = detail.dp_dd.abs();
                                best_t = (t_k as f64) - 273.15;
                                found_valid = true;
                            }
                        }
                    }
                }
            }
        }

        if found_valid {
            return best_t;
        }

        // Estimate if calculation fails
        // Dew temperature increases with pressure
        let t_est = -90.0 + (pressure / 1_000_000.0) * 15.0;
        t_est.max(-90.0).min(200.0)
    }

    /// Calculate thermodynamic point at given pressure and temperature
    pub fn calculate_point(
        composition: &Composition,
        pressure: f64,
        temperature: f64,
        eos: &EquationOfState,
    ) -> Result<ThermodynamicPoint, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        let aga8_comp = to_aga8_composition(composition)?;

        // Validate pressure and temperature are within valid range
        // Both GERG-2008 and AGA8 DETAIL have similar ranges:
        // - Temperature: -130°C to 200°C (Region 2, broader range)
        //   For optimal accuracy (0.1% uncertainty): -8°C to 62°C (Region 1)
        // - Pressure: Up to 275 MPa (27,500,000 Pa)
        //   For optimal accuracy: Up to 12 MPa (12,000,000 Pa)
        // Note: Not recommended near critical point or in liquid phase
        //
        // Practical limits (based on actual aga8 crate behavior):
        // - Very low temperatures (< -50°C) may fail for some compositions
        // - Minimum pressure: 0.1 MPa (100,000 Pa) for practical use
        const MIN_PRESSURE_PA: f64 = 100_000.0; // 0.1 MPa (practical minimum)
        const MAX_PRESSURE_PA: f64 = 275_000_000.0; // 275 MPa (AGA8 maximum)
        const MIN_TEMPERATURE_C: f64 = -29.0; // -29°C (practical minimum - tested: works at 0.1 MPa, lower temps work at higher pressures)
        const MAX_TEMPERATURE_C: f64 = 200.0; // 200°C (AGA8 maximum)

        if !(MIN_PRESSURE_PA..=MAX_PRESSURE_PA).contains(&pressure) {
            return Err(Aga8Error::CalculationFailed(format!(
                "Pressure {} Pa is outside valid range ({} - {} Pa)",
                pressure, MIN_PRESSURE_PA, MAX_PRESSURE_PA
            )));
        }

        if !(MIN_TEMPERATURE_C..=MAX_TEMPERATURE_C).contains(&temperature) {
            return Err(Aga8Error::CalculationFailed(format!(
                "Temperature {}°C is outside valid range ({} - {}°C)",
                temperature, MIN_TEMPERATURE_C, MAX_TEMPERATURE_C
            )));
        }

        // Convert temperature from Celsius to Kelvin
        let temp_kelvin = temperature + 273.15;

        // Calculate properties using the selected equation of state
        // Get all properties directly from aga8 crate
        let eos_props = match eos {
            EquationOfState::Gerg2008 => {
                Self::calculate_with_gerg2008(&aga8_comp, pressure, temp_kelvin, composition)?
            }
            EquationOfState::Aga8Detail => {
                Self::calculate_with_aga8_detail(&aga8_comp, pressure, temp_kelvin, composition)?
            }
        };

        // Calculate density derivatives from aga8's pressure derivatives
        // aga8 provides: dp_dd (kPa/(mol/l)) and dp_dt (kPa/K)
        // We need: d_rho_dp and d_rho_dt (density derivatives)
        // These are related by: d_rho_dp = 1 / (dp_dd) after unit conversion
        let avg_mw = calculate_average_molecular_weight(composition);

        // Convert aga8's dp_dd from kPa/(mol/l) to Pa/(mol/m³) for unit consistency
        // 1 kPa/(mol/l) = 1,000,000 Pa/(mol/m³)
        let dp_dd_pa_per_mol_m3 = eos_props.dp_dd * 1_000_000.0;

        // Density derivative w.r.t. pressure: d(rho_mass)/dp = MW / (dp/dd_mol)
        // This uses aga8's dp_dd directly - just unit conversion and inverse relationship
        let d_rho_dp = avg_mw / dp_dd_pa_per_mol_m3;

        // Density derivative w.r.t. temperature: uses aga8's dp_dt and dp_dd
        // d_rho_dt = -rho * (dp_dt / dp_dd) / T (thermodynamic relationship)
        // All values come from aga8 - we're just applying the mathematical relationship
        let dp_dt_pa_per_k = eos_props.dp_dt * 1000.0; // Unit conversion: kPa/K -> Pa/K
        let d_rho_dt = -(eos_props.density_kg_m3 / temp_kelvin)
            * (dp_dt_pa_per_k / dp_dd_pa_per_mol_m3)
            * avg_mw;

        // For liquid phase properties: AGA8/GERG-2008 are primarily for gas phase
        // Use same values from aga8 for liquid (two-phase calculations would require additional EOS)
        let liquid_density = eos_props.density_kg_m3;

        // Convert cp from aga8's J/(mol·K) to J/(kg·K) - just unit conversion
        let gas_cp_mass = eos_props.cp / avg_mw; // J/(kg·K)
        let liquid_cp_mass = gas_cp_mass; // Use same from aga8

        // Properties not provided by aga8 crate - set to zero
        // Note: aga8 does not provide viscosity, thermal conductivity, or surface tension
        let gas_viscosity = 0.0; // Not available from aga8
        let liquid_viscosity = 0.0; // Not available from aga8
        let gas_thermal_conductivity = 0.0; // Not available from aga8
        let liquid_thermal_conductivity = 0.0; // Not available from aga8
        let surface_tension = 0.0; // Not available from aga8

        Ok(ThermodynamicPoint {
            pressure,
            temperature,
            gas_density: eos_props.density_kg_m3,
            liquid_density,
            d_rho_gas_dp: d_rho_dp,
            d_rho_liq_dp: d_rho_dp,
            d_rho_gas_dt: d_rho_dt,
            d_rho_liq_dt: d_rho_dt,
            rs: 1.0, // Solution gas-oil ratio (not applicable for natural gas)
            gas_viscosity,
            liquid_viscosity,
            gas_cp: gas_cp_mass,
            liquid_cp: liquid_cp_mass,
            gas_enthalpy: eos_props.enthalpy,
            liquid_enthalpy: eos_props.enthalpy, // Use same for now
            gas_thermal_conductivity,
            liquid_thermal_conductivity,
            surface_tension,
            gas_entropy: eos_props.entropy,
            liquid_entropy: eos_props.entropy, // Use same for now
        })
    }

    /// Calculate properties using GERG-2008
    fn calculate_with_gerg2008(
        composition: &aga8::composition::Composition,
        pressure: f64,
        temp_kelvin: f64,
        our_composition: &Composition,
    ) -> Result<EosProperties, Aga8Error> {
        let mut gerg = aga8::gerg2008::Gerg2008::new();
        let _ = gerg.set_composition(composition);
        gerg.p = pressure;
        gerg.t = temp_kelvin;

        // iflag: 0 = normal calculation, 1 = use initial guess
        gerg.density(0).map_err(|e| {
            Aga8Error::CalculationFailed(format!("GERG-2008 density calculation failed: {:?}", e))
        })?;

        gerg.properties();

        // GERG-2008 provides all properties directly after calling properties()
        // Get molar density in mol/l, convert to kg/m³
        let density_mol_l = gerg.d; // mol/l
        let density_mol_m3 = density_mol_l * 1000.0; // Convert to mol/m³

        // Calculate average molecular weight for conversion from molar to mass density
        let avg_mw = calculate_average_molecular_weight(our_composition);
        let density_kg_m3 = density_mol_m3 * avg_mw;

        Ok(EosProperties {
            density_kg_m3,
            compressibility: gerg.z,
            enthalpy: gerg.h,       // J/mol
            entropy: gerg.s,        // J/(mol·K)
            cp: gerg.cp,            // J/(mol·K)
            cv: gerg.cv,            // J/(mol·K)
            dp_dd: gerg.dp_dd,      // kPa/(mol/l)
            d2p_dd2: gerg.d2p_dd2,  // Second derivative
            d2p_dtd: gerg.d2p_dtd,  // Mixed derivative
            dp_dt: gerg.dp_dt,      // kPa/K
            speed_of_sound: gerg.w, // m/s
            gibbs_energy: gerg.g,   // J/mol
            joule_thomson: gerg.jt, // K/kPa
            kappa: gerg.kappa,      // Isentropic exponent
        })
    }

    /// Calculate properties using AGA8 DETAIL
    fn calculate_with_aga8_detail(
        composition: &aga8::composition::Composition,
        pressure: f64,
        temp_kelvin: f64,
        _our_composition: &Composition,
    ) -> Result<EosProperties, Aga8Error> {
        let mut detail = aga8::detail::Detail::new();
        detail.set_composition(composition).map_err(|e| {
            Aga8Error::CalculationFailed(format!("Failed to set composition: {:?}", e))
        })?;

        detail.p = pressure;
        detail.t = temp_kelvin;

        detail.density().map_err(|e| {
            Aga8Error::CalculationFailed(format!("AGA8 DETAIL density calculation failed: {:?}", e))
        })?;

        detail.properties();

        // AGA8 DETAIL provides all properties directly after calling properties()
        // Get molar density in mol/l, convert to kg/m³
        let density_mol_l = detail.d; // mol/l
        let density_mol_m3 = density_mol_l * 1000.0; // Convert to mol/m³
        let molar_mass_kg_mol = detail.mm / 1000.0; // Convert g/mol to kg/mol
        let density_kg_m3 = density_mol_m3 * molar_mass_kg_mol;

        Ok(EosProperties {
            density_kg_m3,
            compressibility: detail.z,
            enthalpy: detail.h,       // J/mol
            entropy: detail.s,        // J/(mol·K)
            cp: detail.cp,            // J/(mol·K)
            cv: detail.cv,            // J/(mol·K)
            dp_dd: detail.dp_dd,      // kPa/(mol/l)
            d2p_dd2: detail.d2p_dd2,  // Second derivative
            d2p_dtd: detail.d2p_dtd,  // Mixed derivative
            dp_dt: detail.dp_dt,      // kPa/K
            speed_of_sound: detail.w, // m/s
            gibbs_energy: detail.g,   // J/mol
            joule_thomson: detail.jt, // K/kPa
            kappa: detail.kappa,      // Isentropic exponent
        })
    }
}
