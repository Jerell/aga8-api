use crate::aga8_calc::{Aga8Calculator, Aga8Error};
use crate::models::{Composition, ThermodynamicData, ThermodynamicDataRequest};
use crate::tab_file::TabFileFormatter;

/// Service layer for generating thermodynamic data
pub struct ThermodynamicService;

impl ThermodynamicService {
    /// Generate complete thermodynamic dataset from request
    pub fn generate_data(
        request: &ThermodynamicDataRequest,
    ) -> Result<ThermodynamicData, Aga8Error> {
        // Validate composition
        request
            .composition
            .validate()
            .map_err(Aga8Error::InvalidComposition)?;

        // Generate grids
        let pressure_grid = request.pressure_range.generate_grid();
        let enthalpy_grid = request.enthalpy_range.generate_grid();

        // Calculate critical point
        let critical_point = Aga8Calculator::calculate_critical_point(&request.composition)?;

        // For temperature grid, we'll need to convert from enthalpy
        // This is a simplified approach - in reality, we'd need to solve for temperature
        // given enthalpy at each pressure point
        let temperature_grid = Self::generate_temperature_grid_from_enthalpy(
            &request.composition,
            &pressure_grid,
            &enthalpy_grid,
        )?;

        // Calculate phase boundaries
        let phase_boundaries = Aga8Calculator::calculate_phase_boundaries(
            &request.composition,
            &temperature_grid,
            &pressure_grid,
        )?;

        // Generate thermodynamic points for the grid
        let mut points = Vec::new();
        for &pressure in &pressure_grid {
            for &temperature in &temperature_grid {
                let point =
                    Aga8Calculator::calculate_point(&request.composition, pressure, temperature)?;
                points.push(point);
            }
        }

        Ok(ThermodynamicData {
            composition: request.composition.clone(),
            critical_point,
            phase_boundaries,
            points,
            pressure_grid,
            temperature_grid,
        })
    }

    /// Generate tab file content from thermodynamic data
    pub fn generate_tab_file(data: &ThermodynamicData) -> String {
        TabFileFormatter::format(
            &data.composition,
            &data.critical_point,
            &data.phase_boundaries,
            &data.pressure_grid,
            &data.temperature_grid,
            &data.points,
        )
    }

    /// Generate temperature grid from enthalpy range
    /// This is a simplified implementation - in practice, you'd need to solve
    /// for temperature given enthalpy at each pressure point
    fn generate_temperature_grid_from_enthalpy(
        _composition: &Composition,
        _pressure_grid: &[f64],
        enthalpy_grid: &[f64],
    ) -> Result<Vec<f64>, Aga8Error> {
        // For now, create a temperature grid based on a reasonable range
        // In a real implementation, you'd solve T = f(H, P) for each enthalpy value
        // This is a placeholder that creates a temperature grid
        let min_temp = -20.0; // °C
        let max_temp = 35.0; // °C
        let num_points = enthalpy_grid.len();

        if num_points == 1 {
            return Ok(vec![min_temp]);
        }

        let step = (max_temp - min_temp) / (num_points - 1) as f64;
        Ok((0..num_points)
            .map(|i| min_temp + step * i as f64)
            .collect())
    }
}
