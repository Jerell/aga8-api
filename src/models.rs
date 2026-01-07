use poem_openapi::Object;
use serde::{Deserialize, Serialize};

/// Gas composition with mole fractions
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct Composition {
    /// Component names (e.g., ["CO2", "H2"])
    pub components: Vec<String>,
    /// Mole fractions (must sum to 1.0)
    pub mole_fractions: Vec<f64>,
}

/// Pressure range specification
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct PressureRange {
    /// Minimum pressure in Pa
    pub min: f64,
    /// Maximum pressure in Pa
    pub max: f64,
    /// Number of points in the grid
    pub points: usize,
}

/// Enthalpy range specification
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct EnthalpyRange {
    /// Minimum enthalpy in J/mol
    pub min: f64,
    /// Maximum enthalpy in J/mol
    pub max: f64,
    /// Number of points in the grid
    pub points: usize,
}

/// Equation of state to use for calculations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EquationOfState {
    /// GERG-2008 equation of state (default)
    #[default]
    Gerg2008,
    /// AGA8 DETAIL equation of state
    Aga8Detail,
}

impl From<&str> for EquationOfState {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "aga8detail" | "aga8_detail" | "detail" => EquationOfState::Aga8Detail,
            _ => EquationOfState::Gerg2008, // Default to GERG-2008
        }
    }
}

/// Request for generating thermodynamic data
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct ThermodynamicDataRequest {
    /// Gas composition
    pub composition: Composition,
    /// Pressure range
    pub pressure_range: PressureRange,
    /// Enthalpy range
    pub enthalpy_range: EnthalpyRange,
    /// Equation of state to use: "gerg2008" (default) or "aga8detail"
    #[serde(default = "default_eos_string")]
    #[oai(default = "default_eos_string")]
    pub equation_of_state: String,
}

fn default_eos_string() -> String {
    "gerg2008".to_string()
}

impl ThermodynamicDataRequest {
    /// Get the equation of state enum from the string field
    pub fn eos(&self) -> EquationOfState {
        EquationOfState::from(self.equation_of_state.as_str())
    }
}

/// Critical point data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalPoint {
    /// Critical pressure in Pa
    pub pressure: f64,
    /// Critical temperature in °C
    pub temperature: f64,
}

/// Phase boundary data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseBoundaries {
    /// Bubble point pressures at each temperature (Pa)
    pub bubble_pressures: Vec<f64>,
    /// Bubble point temperatures at each pressure (°C)
    pub bubble_temperatures: Vec<f64>,
    /// Dew point pressures at each temperature (Pa)
    pub dew_pressures: Vec<f64>,
    /// Dew point temperatures at each pressure (°C)
    pub dew_temperatures: Vec<f64>,
}

/// Single thermodynamic data point
#[derive(Debug, Clone)]
pub struct ThermodynamicPoint {
    /// Pressure in Pa
    pub pressure: f64,
    /// Temperature in °C
    pub temperature: f64,
    /// Gas density in kg/m³
    pub gas_density: f64,
    /// Liquid density in kg/m³
    pub liquid_density: f64,
    /// Gas density derivative with respect to pressure
    pub d_rho_gas_dp: f64,
    /// Liquid density derivative with respect to pressure
    pub d_rho_liq_dp: f64,
    /// Gas density derivative with respect to temperature
    pub d_rho_gas_dt: f64,
    /// Liquid density derivative with respect to temperature
    pub d_rho_liq_dt: f64,
    /// Solution gas-oil ratio
    pub rs: f64,
    /// Gas viscosity in Pa·s
    pub gas_viscosity: f64,
    /// Liquid viscosity in Pa·s
    pub liquid_viscosity: f64,
    /// Gas heat capacity in J/(kg·K)
    pub gas_cp: f64,
    /// Liquid heat capacity in J/(kg·K)
    pub liquid_cp: f64,
    /// Gas enthalpy in J/mol
    pub gas_enthalpy: f64,
    /// Liquid enthalpy in J/mol
    pub liquid_enthalpy: f64,
    /// Gas thermal conductivity in W/(m·K)
    pub gas_thermal_conductivity: f64,
    /// Liquid thermal conductivity in W/(m·K)
    pub liquid_thermal_conductivity: f64,
    /// Surface tension in N/m
    pub surface_tension: f64,
    /// Gas entropy in J/(mol·K)
    pub gas_entropy: f64,
    /// Liquid entropy in J/(mol·K)
    pub liquid_entropy: f64,
}

/// Complete thermodynamic dataset
#[derive(Debug, Clone)]
pub struct ThermodynamicData {
    /// Composition information
    pub composition: Composition,
    /// Critical point
    pub critical_point: CriticalPoint,
    /// Phase boundaries
    pub phase_boundaries: PhaseBoundaries,
    /// Grid of thermodynamic points
    pub points: Vec<ThermodynamicPoint>,
    /// Pressure grid values
    pub pressure_grid: Vec<f64>,
    /// Temperature grid values
    pub temperature_grid: Vec<f64>,
    /// Equation of state used
    pub equation_of_state: EquationOfState,
}

impl Composition {
    /// Validate that mole fractions sum to 1.0
    pub fn validate(&self) -> Result<(), String> {
        if self.components.len() != self.mole_fractions.len() {
            return Err("Components and mole fractions must have the same length".to_string());
        }
        if self.components.is_empty() {
            return Err("Composition must have at least one component".to_string());
        }
        let sum: f64 = self.mole_fractions.iter().sum();
        if (sum - 1.0).abs() > 1e-6 {
            return Err(format!("Mole fractions must sum to 1.0, got {}", sum));
        }
        Ok(())
    }
}

impl PressureRange {
    /// Validate pressure range is within AGA8 valid limits
    /// AGA8 (GERG-2008) valid pressure range: 0.1 MPa to 275 MPa
    pub fn validate(&self) -> Result<(), String> {
        const MIN_PRESSURE_PA: f64 = 100_000.0; // 0.1 MPa (practical minimum)
        const MAX_PRESSURE_PA: f64 = 275_000_000.0; // 275 MPa (AGA8 maximum)

        if self.min < MIN_PRESSURE_PA {
            return Err(format!(
                "Minimum pressure {} Pa is below AGA8 valid range (minimum: {} Pa)",
                self.min, MIN_PRESSURE_PA
            ));
        }
        if self.max > MAX_PRESSURE_PA {
            return Err(format!(
                "Maximum pressure {} Pa is above AGA8 valid range (maximum: {} Pa)",
                self.max, MAX_PRESSURE_PA
            ));
        }
        if self.min >= self.max {
            return Err("Minimum pressure must be less than maximum pressure".to_string());
        }
        if self.points == 0 {
            return Err("Number of points must be greater than 0".to_string());
        }
        Ok(())
    }

    /// Generate pressure grid
    pub fn generate_grid(&self) -> Vec<f64> {
        if self.points == 1 {
            return vec![self.min];
        }
        let step = (self.max - self.min) / (self.points - 1) as f64;
        (0..self.points)
            .map(|i| self.min + step * i as f64)
            .collect()
    }
}

impl EnthalpyRange {
    /// Generate enthalpy grid
    pub fn generate_grid(&self) -> Vec<f64> {
        if self.points == 1 {
            return vec![self.min];
        }
        let step = (self.max - self.min) / (self.points - 1) as f64;
        (0..self.points)
            .map(|i| self.min + step * i as f64)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composition_validation() {
        let valid = Composition {
            components: vec!["CO2".to_string(), "H2".to_string()],
            mole_fractions: vec![0.99, 0.01],
        };
        assert!(valid.validate().is_ok());

        let invalid_sum = Composition {
            components: vec!["CO2".to_string(), "H2".to_string()],
            mole_fractions: vec![0.99, 0.02],
        };
        assert!(invalid_sum.validate().is_err());

        let invalid_length = Composition {
            components: vec!["CO2".to_string(), "H2".to_string()],
            mole_fractions: vec![1.0],
        };
        assert!(invalid_length.validate().is_err());
    }

    #[test]
    fn test_pressure_range_grid() {
        let range = PressureRange {
            min: 100_000.0,
            max: 200_000.0,
            points: 3,
        };
        let grid = range.generate_grid();
        assert_eq!(grid.len(), 3);
        assert!((grid[0] - 100_000.0).abs() < 1e-6);
        assert!((grid[2] - 200_000.0).abs() < 1e-6);
    }

    #[test]
    fn test_enthalpy_range_grid() {
        let range = EnthalpyRange {
            min: -100_000.0,
            max: 100_000.0,
            points: 5,
        };
        let grid = range.generate_grid();
        assert_eq!(grid.len(), 5);
        assert!((grid[0] - (-100_000.0)).abs() < 1e-6);
        assert!((grid[4] - 100_000.0).abs() < 1e-6);
    }
}
