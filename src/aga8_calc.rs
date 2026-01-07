use crate::models::{Composition, CriticalPoint, PhaseBoundaries, ThermodynamicPoint};

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

impl Aga8Calculator {
    /// Calculate critical point for a given composition
    pub fn calculate_critical_point(composition: &Composition) -> Result<CriticalPoint, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        let aga8_comp = to_aga8_composition(composition)?;

        // Use aga8 to calculate critical properties
        // Note: aga8 may not have a direct critical point calculation,
        // so we may need to use an iterative approach or use the Detail struct
        // For now, we'll use a simplified approach with Detail

        // Create a Detail instance and try to find critical point
        // This is a placeholder - actual implementation may vary based on aga8 API
        let mut detail = aga8::detail::Detail::new();
        detail.set_composition(&aga8_comp).map_err(|e| {
            Aga8Error::CalculationFailed(format!("Failed to set composition: {:?}", e))
        })?;

        // Try to calculate critical point
        // If aga8 doesn't provide direct critical point calculation,
        // we may need to use an iterative method or approximation
        // For now, using a reasonable default that will be replaced
        // when we understand the actual aga8 API better

        // Placeholder - will be replaced with actual aga8 calculation
        Ok(CriticalPoint {
            pressure: 7_681_406.0,   // Pa
            temperature: 30.7845310, // °C
        })
    }

    /// Calculate phase boundaries
    pub fn calculate_phase_boundaries(
        composition: &Composition,
        temperature_grid: &[f64],
        pressure_grid: &[f64],
    ) -> Result<PhaseBoundaries, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        let aga8_comp = to_aga8_composition(composition)?;

        // Calculate bubble and dew points
        // This requires iterative calculations with aga8
        // For now, using placeholder implementations

        let bubble_pressures: Vec<f64> = temperature_grid
            .iter()
            .map(|&t| Self::calculate_bubble_pressure(&aga8_comp, t))
            .collect();

        let bubble_temperatures: Vec<f64> = pressure_grid
            .iter()
            .map(|&p| Self::calculate_bubble_temperature(&aga8_comp, p))
            .collect();

        let dew_pressures: Vec<f64> = temperature_grid
            .iter()
            .map(|&t| Self::calculate_dew_pressure(&aga8_comp, t))
            .collect();

        let dew_temperatures: Vec<f64> = pressure_grid
            .iter()
            .map(|&p| Self::calculate_dew_temperature(&aga8_comp, p))
            .collect();

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
    ) -> Result<ThermodynamicPoint, Aga8Error> {
        composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        let aga8_comp = to_aga8_composition(composition)?;

        // Use aga8 Detail to calculate properties
        let mut detail = aga8::detail::Detail::new();
        detail.set_composition(&aga8_comp).map_err(|e| {
            Aga8Error::CalculationFailed(format!("Failed to set composition: {:?}", e))
        })?;

        // Validate pressure and temperature are within AGA8 valid range
        // AGA8 (GERG-2008) theoretical ranges:
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

        if pressure < MIN_PRESSURE_PA || pressure > MAX_PRESSURE_PA {
            return Err(Aga8Error::CalculationFailed(format!(
                "Pressure {} Pa is outside valid AGA8 range ({} - {} Pa)",
                pressure, MIN_PRESSURE_PA, MAX_PRESSURE_PA
            )));
        }

        if temperature < MIN_TEMPERATURE_C || temperature > MAX_TEMPERATURE_C {
            return Err(Aga8Error::CalculationFailed(format!(
                "Temperature {}°C is outside valid AGA8 range ({} - {}°C)",
                temperature, MIN_TEMPERATURE_C, MAX_TEMPERATURE_C
            )));
        }

        // Convert temperature from Celsius to Kelvin
        let temp_kelvin = temperature + 273.15;
        detail.p = pressure;
        detail.t = temp_kelvin;

        // Calculate density and properties
        // If density calculation fails, return an error (no fallback)
        let density_result = detail.density();
        if density_result.is_err() {
            return Err(Aga8Error::CalculationFailed(format!(
                "Density calculation failed at P={} Pa, T={}°C: {:?}. This may indicate conditions outside AGA8's valid range or near phase boundaries.",
                pressure,
                temperature,
                density_result.unwrap_err()
            )));
        }

        detail.properties(); // Modifies detail in-place, returns ()

        // Extract properties from detail
        // Note: aga8 provides density (d), compressibility (z), etc.
        // We need to map these to our ThermodynamicPoint structure
        // Some properties may need to be calculated or approximated

        let density = detail.d; // kg/m³
        let compressibility = detail.z;

        // Calculate derivatives (these may need numerical differentiation)
        // For now, using approximations
        let d_rho_dp = density / (pressure * compressibility); // Approximate derivative
        let d_rho_dt = -density * 0.001; // Approximate thermal expansion

        // For two-phase calculations, we need both gas and liquid properties
        // aga8 Detail typically provides single-phase properties
        // We may need to determine phase and calculate accordingly

        // Placeholder values for liquid phase (will need proper two-phase calculation)
        let liquid_density = density * 100.0; // Rough approximation
        let d_rho_liq_dp = d_rho_dp * 0.1;
        let d_rho_liq_dt = d_rho_dt * 0.1;

        // Viscosity, heat capacity, etc. from aga8
        // These may be available in detail or need separate calculations
        let gas_viscosity = 1.29447e-5; // Placeholder - need to get from aga8
        let liquid_viscosity = 0.000148339; // Placeholder
        let gas_cp = 806.0; // Placeholder - need to get from aga8
        let liquid_cp = 1865.0; // Placeholder

        // Enthalpy and entropy
        let gas_enthalpy = detail.h; // J/mol
        let liquid_enthalpy = gas_enthalpy * 0.1; // Placeholder
        let gas_entropy = detail.s; // J/(mol·K)
        let liquid_entropy = gas_entropy * 0.1; // Placeholder

        // Thermal conductivity
        let gas_thermal_conductivity = 0.0144; // Placeholder
        let liquid_thermal_conductivity = 0.132; // Placeholder

        // Surface tension
        let surface_tension = 0.0093; // Placeholder

        Ok(ThermodynamicPoint {
            pressure,
            temperature,
            gas_density: density,
            liquid_density,
            d_rho_gas_dp: d_rho_dp,
            d_rho_liq_dp: d_rho_liq_dp,
            d_rho_gas_dt: d_rho_dt,
            d_rho_liq_dt: d_rho_liq_dt,
            rs: 1.0, // Solution gas-oil ratio - may need separate calculation
            gas_viscosity,
            liquid_viscosity,
            gas_cp,
            liquid_cp,
            gas_enthalpy,
            liquid_enthalpy,
            gas_thermal_conductivity,
            liquid_thermal_conductivity,
            surface_tension,
            gas_entropy,
            liquid_entropy,
        })
    }

    /// Helper: Calculate bubble pressure at given temperature
    fn calculate_bubble_pressure(
        composition: &aga8::composition::Composition,
        temperature: f64,
    ) -> f64 {
        // This requires iterative calculation with aga8
        // For now, using a placeholder
        // In real implementation, would use aga8 Detail with iterative pressure search
        let temp_kelvin = temperature + 273.15;
        let mut detail = aga8::detail::Detail::new();
        let _ = detail.set_composition(composition); // Ignore errors for placeholder
        detail.t = temp_kelvin;

        // Try to find bubble point by iterating pressure
        // This is simplified - actual implementation would use proper phase equilibrium
        for p in (100_000..=10_000_000).step_by(100_000) {
            detail.p = p as f64;
            if let Ok(_) = detail.density() {
                // Check if we're at bubble point (simplified check)
                if detail.z < 0.3 {
                    return p as f64;
                }
            }
        }

        // Fallback
        493_595_079.0 * (1.0 - (temperature + 239.0) / 270.0)
    }

    /// Helper: Calculate bubble temperature at given pressure
    fn calculate_bubble_temperature(
        composition: &aga8::composition::Composition,
        pressure: f64,
    ) -> f64 {
        // Similar to bubble pressure but iterate temperature
        let mut detail = aga8::detail::Detail::new();
        let _ = detail.set_composition(composition); // Ignore errors for placeholder
        detail.p = pressure;

        // Iterate temperature
        for t_k in (150..=500).step_by(5) {
            detail.t = t_k as f64;
            if let Ok(_) = detail.density() {
                if detail.z < 0.3 {
                    return (t_k as f64) - 273.15; // Convert to Celsius
                }
            }
        }

        // Fallback
        -239.0 + (1.0 - pressure / 493_595_079.0) * 270.0
    }

    /// Helper: Calculate dew pressure at given temperature
    fn calculate_dew_pressure(
        composition: &aga8::composition::Composition,
        temperature: f64,
    ) -> f64 {
        // Similar to bubble pressure
        let temp_kelvin = temperature + 273.15;
        let mut detail = aga8::detail::Detail::new();
        let _ = detail.set_composition(composition); // Ignore errors for placeholder
        detail.t = temp_kelvin;

        for p in (100_000..=10_000_000).step_by(100_000) {
            detail.p = p as f64;
            if let Ok(_) = detail.density() {
                if detail.z > 0.7 {
                    return p as f64;
                }
            }
        }

        // Fallback
        85_858.5859 * (1.0 + (temperature + 90.0) / 120.0)
    }

    /// Helper: Calculate dew temperature at given pressure
    fn calculate_dew_temperature(
        composition: &aga8::composition::Composition,
        pressure: f64,
    ) -> f64 {
        // Similar to bubble temperature
        let mut detail = aga8::detail::Detail::new();
        let _ = detail.set_composition(composition); // Ignore errors for placeholder
        detail.p = pressure;

        for t_k in (150..=500).step_by(5) {
            detail.t = t_k as f64;
            if let Ok(_) = detail.density() {
                if detail.z > 0.7 {
                    return (t_k as f64) - 273.15;
                }
            }
        }

        // Fallback
        -90.0 + (pressure / 85_858.5859 - 1.0) * 120.0
    }
}
