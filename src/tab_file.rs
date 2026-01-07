use crate::models::{Composition, CriticalPoint, PhaseBoundaries, ThermodynamicPoint};

/// Format thermodynamic data as a Multiflash tab file
pub struct TabFileFormatter;

impl TabFileFormatter {
    /// Generate the complete tab file content
    pub fn format(
        composition: &Composition,
        critical_point: &CriticalPoint,
        phase_boundaries: &PhaseBoundaries,
        pressure_grid: &[f64],
        temperature_grid: &[f64],
        points: &[ThermodynamicPoint],
    ) -> String {
        let mut output = String::new();

        // Header
        let component_list = composition
            .components
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
            .join(" , ");
        let label = format!("AGA8_{}", component_list.replace(" ", "_"));
        output.push_str(&format!("PVTTABLE LABEL = {} ,  PHASE = TWO, \\\n", label));
        output.push_str("!\n");
        output.push_str("! Multiflash Version 7.4.08     May 2023\n");
        output.push_str("!\n");
        output.push_str("! EOS:  AGA8 - GERG-2008                                                                          !\n");
        output.push_str("!\n");

        // Components
        output.push_str(&format!("COMPONENTS = ( {} ), \\\n", component_list));

        // Mole fractions
        let mole_str = composition
            .mole_fractions
            .iter()
            .map(|&m| format!("{:.9}", m))
            .collect::<Vec<_>>()
            .join(" , ");
        output.push_str(&format!("MOLES = ( {} ), \\\n", mole_str));

        // Molecular weights (placeholder - should come from component database)
        let mw_str = composition
            .components
            .iter()
            .map(|_| "44.0098000") // Placeholder - should look up actual MW
            .collect::<Vec<_>>()
            .join(" , ");
        output.push_str(&format!("MOLWEIGHT = ( {} ), \\\n", mw_str));

        // Standard conditions
        output.push_str("STDPRESSURE = 101325 PA , \\\n");
        output.push_str("STDTEMPERATURE = 15.55556 C, \\\n");
        output.push_str("GOR = -999 SM3/SM3, \\\n");
        output.push_str("GLR = -999 SM3/SM3, \\\n");
        output.push_str("WC = -999 , \\\n");
        output.push_str("STDGASDENSITY = 1.85037281 KG/M3, \\\n");
        output.push_str("STDOILDENSITY = -999 KG/M3, \\\n");
        output.push_str("STDWATDENSITY = -999 KG/M3, \\\n");
        output.push_str("MESHTYPE = STANDARD, \\\n");

        // Pressure grid
        output.push_str(&Self::format_array("PRESSURE", pressure_grid, "PA"));

        // Temperature grid
        output.push_str(&Self::format_array("TEMPERATURE", temperature_grid, "C"));

        // Bubble pressures
        output.push_str(&Self::format_array(
            "BUBBLEPRESSURES",
            &phase_boundaries.bubble_pressures,
            "PA",
        ));

        // Bubble temperatures
        output.push_str(&Self::format_array(
            "BUBBLETEMPERATURES",
            &phase_boundaries.bubble_temperatures,
            "C",
        ));

        // Dew pressures
        output.push_str(&Self::format_array(
            "DEWPRESSURES",
            &phase_boundaries.dew_pressures,
            "PA",
        ));

        // Dew temperatures
        output.push_str(&Self::format_array(
            "DEWTEMPERATURES",
            &phase_boundaries.dew_temperatures,
            "C",
        ));

        // Critical point
        output.push_str(&format!(
            "CRITICALPRESSURE = ( {:.2} ) PA, \\\n",
            critical_point.pressure
        ));
        output.push_str(&format!(
            "CRITICALTEMPERATURE = ( {:.7} ) C, \\\n",
            critical_point.temperature
        ));

        // Columns
        output.push_str("COLUMNS = ( PT, TM, ROG, ROHL, DROGDP,  DROHLDP, DROGDT, DROHLDT, RS, VISG, VISHL, CPG, CPHL,   HG, HHL, TCG, TCHL, SIGGHL, SEG, SEHL)\n");

        // Data points
        for point in points {
            output.push_str(&Self::format_point(point));
        }

        output
    }

    /// Format an array of values for the tab file
    fn format_array(name: &str, values: &[f64], unit: &str) -> String {
        let mut output = format!("{} = ( ", name);
        let line_width = 80;
        let mut current_line_len = output.len();

        for (i, &value) in values.iter().enumerate() {
            let value_str = if value.abs() < 1.0 {
                format!("{:.9}", value)
            } else if value.abs() < 1000.0 {
                format!("{:.2}", value)
            } else {
                format!("{:.2}", value)
            };

            let entry = if i == values.len() - 1 {
                format!("{} ) {}, \\\n", value_str, unit)
            } else {
                format!("{} , ", value_str)
            };

            // Check if we need a line break
            if current_line_len + entry.len() > line_width && i > 0 {
                output.push_str("\\\n");
                current_line_len = 0;
            }

            output.push_str(&entry);
            current_line_len += entry.len();
        }

        output
    }

    /// Format a single thermodynamic point
    fn format_point(point: &ThermodynamicPoint) -> String {
        format!(
            "PVTTABLE POINT = ( {:.2} , {:.9} , {:.8} , {:.5} , {:.8} , {:.8} , {:.8} , {:.7} , {:.8} , {:.8} , {:.9} , {:.6} , {:.5} ,   {:.3} , {:.2} , {:.9} , {:.9} , {:.9} , {:.6} , {:.4} )\n",
            point.pressure,
            point.temperature,
            point.gas_density,
            point.liquid_density,
            point.d_rho_gas_dp,
            point.d_rho_liq_dp,
            point.d_rho_gas_dt,
            point.d_rho_liq_dt,
            point.rs,
            point.gas_viscosity,
            point.liquid_viscosity,
            point.gas_cp,
            point.liquid_cp,
            point.gas_enthalpy,
            point.liquid_enthalpy,
            point.gas_thermal_conductivity,
            point.liquid_thermal_conductivity,
            point.surface_tension,
            point.gas_entropy,
            point.liquid_entropy,
        )
    }
}
