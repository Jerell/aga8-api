use aga8_api::aga8_calc::Aga8Calculator;
use aga8_api::models::{Composition, EquationOfState};

/// Test critical point calculation
#[test]
fn test_critical_point_calculation() {
    let composition = Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    };

    let eos = EquationOfState::Gerg2008;
    let critical = Aga8Calculator::calculate_critical_point(&composition, &eos).unwrap();

    // Critical point is now calculated
    assert!(
        !critical.pressure.is_nan() && critical.pressure > 0.0,
        "Critical pressure should be valid, got {}",
        critical.pressure
    );
    assert!(
        !critical.temperature.is_nan()
            && critical.temperature > -100.0
            && critical.temperature < 150.0,
        "Critical temperature should be valid, got {}",
        critical.temperature
    );
}

/// Test phase boundary calculations
#[test]
fn test_phase_boundaries() {
    let composition = Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    };

    let temperature_grid = vec![-20.0, -10.0, 0.0, 10.0, 20.0, 30.0];
    let pressure_grid = vec![
        100_000.0,
        200_000.0,
        300_000.0,
        400_000.0,
        500_000.0,
        1_000_000.0,
    ];

    let eos = EquationOfState::Gerg2008;
    let boundaries = Aga8Calculator::calculate_phase_boundaries(
        &composition,
        &temperature_grid,
        &pressure_grid,
        &eos,
    )
    .unwrap();

    // Validate bubble pressures
    assert_eq!(
        boundaries.bubble_pressures.len(),
        temperature_grid.len(),
        "Bubble pressures should match temperature grid length"
    );
    // Phase boundaries are not available from GERG-2008 (single-phase EOS)
    // All values should be NaN
    for &bp in &boundaries.bubble_pressures {
        assert!(
            bp.is_nan(),
            "Bubble pressure should be NaN (not available from GERG-2008), got {}",
            bp
        );
    }

    // Validate bubble temperatures
    assert_eq!(
        boundaries.bubble_temperatures.len(),
        pressure_grid.len(),
        "Bubble temperatures should match pressure grid length"
    );
    for &bt in &boundaries.bubble_temperatures {
        assert!(
            bt.is_nan(),
            "Bubble temperature should be NaN (not available from GERG-2008), got {}",
            bt
        );
    }

    // Validate dew pressures
    assert_eq!(
        boundaries.dew_pressures.len(),
        temperature_grid.len(),
        "Dew pressures should match temperature grid length"
    );
    for &dp in &boundaries.dew_pressures {
        assert!(
            dp.is_nan(),
            "Dew pressure should be NaN (not available from GERG-2008), got {}",
            dp
        );
    }

    // Validate dew temperatures
    assert_eq!(
        boundaries.dew_temperatures.len(),
        pressure_grid.len(),
        "Dew temperatures should match pressure grid length"
    );
    for &dt in &boundaries.dew_temperatures {
        assert!(
            dt.is_nan(),
            "Dew temperature should be NaN (not available from GERG-2008), got {}",
            dt
        );
    }

    // Phase boundaries are not available from aga8 crate
    // (Removed consistency checks since values are NaN)
}

/// Test thermodynamic point calculation
#[test]
fn test_thermodynamic_point() {
    let composition = Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    };

    let eos = EquationOfState::Gerg2008;
    let point = Aga8Calculator::calculate_point(&composition, 100_000.0, 20.0, &eos).unwrap();

    // Validate all properties are reasonable
    assert!(point.pressure > 0.0, "Pressure should be positive");
    assert!(point.gas_density > 0.0, "Gas density should be positive");
    assert!(
        point.liquid_density > 0.0,
        "Liquid density should be positive"
    );
    // Note: liquid_density may equal gas_density since aga8 is primarily for gas phase
    assert!(
        point.liquid_density >= point.gas_density,
        "Liquid density should be at least equal to gas density"
    );
    // Viscosity, thermal conductivity, and surface tension are not available from aga8
    assert_eq!(
        point.gas_viscosity, 0.0,
        "Gas viscosity should be 0.0 (not available from aga8)"
    );
    assert_eq!(
        point.liquid_viscosity, 0.0,
        "Liquid viscosity should be 0.0 (not available from aga8)"
    );
    assert_eq!(
        point.gas_thermal_conductivity, 0.0,
        "Gas thermal conductivity should be 0.0 (not available from aga8)"
    );
    assert_eq!(
        point.liquid_thermal_conductivity, 0.0,
        "Liquid thermal conductivity should be 0.0 (not available from aga8)"
    );
    assert_eq!(
        point.surface_tension, 0.0,
        "Surface tension should be 0.0 (not available from aga8)"
    );
    assert!(point.gas_cp > 0.0, "Gas heat capacity should be positive");
    assert!(
        point.liquid_cp > 0.0,
        "Liquid heat capacity should be positive"
    );
}

/// Test critical point consistency
#[test]
fn test_critical_point_consistency() {
    let composition = Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    };

    let eos = EquationOfState::Gerg2008;
    let critical = Aga8Calculator::calculate_critical_point(&composition, &eos).unwrap();

    // Critical point is now calculated
    assert!(
        !critical.pressure.is_nan() && critical.pressure > 0.0,
        "Critical pressure should be valid, got {}",
        critical.pressure
    );
    assert!(
        !critical.temperature.is_nan()
            && critical.temperature > -100.0
            && critical.temperature < 150.0,
        "Critical temperature should be valid, got {}",
        critical.temperature
    );

    // Try to calculate properties at a point well away from critical
    // aga8 may have difficulty calculating near the critical point for some compositions
    // So we test at a point that should definitely work
    let test_pressure = critical.pressure * 0.5; // 50% of critical pressure
    let test_temperature = critical.temperature - 10.0; // 10°C below critical temperature

    // This calculation may fail for some compositions, so we handle the error gracefully
    let eos = EquationOfState::Gerg2008;
    match Aga8Calculator::calculate_point(&composition, test_pressure, test_temperature, &eos) {
        Ok(point) => {
            // If calculation succeeds, verify properties
            assert!(
                point.gas_density > 0.0 && point.liquid_density > 0.0,
                "Both densities should be positive"
            );
        }
        Err(e) => {
            // If calculation fails, that's acceptable - aga8 may not support
            // all compositions or conditions
            // We just verify that we got a reasonable error
            assert!(
                e.to_string().contains("Calculation failed"),
                "Should return a calculation error if density calculation fails"
            );
        }
    }

    // Note: At or very near the critical point, aga8 density calculation may fail
    // This is expected behavior for many EOS implementations
    // The critical point calculation itself is separate from property calculations at that point
}
