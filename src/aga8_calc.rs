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
    dp_dt: f64,          // kPa/K
    speed_of_sound: f64, // m/s
    gibbs_energy: f64,   // J/mol
    joule_thomson: f64,  // K/kPa
    kappa: f64,          // Isentropic exponent
}

impl Aga8Calculator {
    /// Calculate critical point for a given composition
    /// Note: aga8 crate does not provide critical point calculations
    /// Returns NaN values to indicate critical point is not available
    pub fn calculate_critical_point(
        composition: &Composition,
        _eos: &EquationOfState,
    ) -> Result<CriticalPoint, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        // aga8 crate does not provide critical point calculations
        // The crate provides pseudocritical properties (for mixing rules) but not
        // the actual critical point of the mixture, which would require solving
        // the criticality conditions: (dp/dd)_T = 0 and (d²p/dd²)_T = 0
        // Return NaN values to indicate these are not available
        Ok(CriticalPoint {
            pressure: f64::NAN,
            temperature: f64::NAN,
        })
    }

    /// Calculate phase boundaries
    /// Note: aga8 crate does not provide phase boundary calculations
    /// Returns placeholder values (NaN) to indicate phase boundaries are not available
    pub fn calculate_phase_boundaries(
        composition: &Composition,
        temperature_grid: &[f64],
        pressure_grid: &[f64],
        _eos: &EquationOfState,
    ) -> Result<PhaseBoundaries, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        // aga8 crate does not provide phase boundary (bubble/dew point) calculations
        // The crate documentation states: "No checks are made to determine the phase boundary"
        // Phase boundaries would require two-phase flash calculations which aga8 does not provide
        // Return NaN values to indicate these are not available
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
            dp_dt: detail.dp_dt,      // kPa/K
            speed_of_sound: detail.w, // m/s
            gibbs_energy: detail.g,   // J/mol
            joule_thomson: detail.jt, // K/kPa
            kappa: detail.kappa,      // Isentropic exponent
        })
    }
}
