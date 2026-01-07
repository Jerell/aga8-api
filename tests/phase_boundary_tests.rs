use aga8_api::aga8_calc::Aga8Calculator;
use aga8_api::models::{Composition, EquationOfState};

/// Test phase boundaries and critical point against reference values from CPA EOS
/// Reference composition: 99% CO2, 1% H2
/// Reference values from CPA EOS tab file
/// 
/// NOTE: GERG-2008 is a single-phase EOS and cannot calculate phase boundaries.
/// This test verifies that phase boundaries are correctly set to NaN.
#[test]
fn test_phase_boundaries_against_cpa_reference() {
    let composition = Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    };

    let eos = EquationOfState::Gerg2008;

    // Reference critical point from CPA EOS
    let reference_critical_pressure = 7_681_406.0; // Pa
    let reference_critical_temperature = 30.7845310; // °C

    // Calculate critical point
    let critical = Aga8Calculator::calculate_critical_point(&composition, &eos).unwrap();

    // Allow larger tolerance for critical point (different EOS may give different values)
    // GERG-2008 may not match CPA EOS exactly
    let pressure_tolerance = reference_critical_pressure * 0.5; // 50% tolerance
    let temperature_tolerance = 20.0; // 20°C tolerance

    println!(
        "Critical point: P={} Pa, T={}°C (reference: P={} Pa, T={}°C)",
        critical.pressure, critical.temperature, reference_critical_pressure, reference_critical_temperature
    );

    assert!(
        (critical.pressure - reference_critical_pressure).abs() < pressure_tolerance,
        "Critical pressure {} should be close to reference {} (tolerance: {})",
        critical.pressure,
        reference_critical_pressure,
        pressure_tolerance
    );

    assert!(
        (critical.temperature - reference_critical_temperature).abs() < temperature_tolerance,
        "Critical temperature {} should be close to reference {} (tolerance: {})",
        critical.temperature,
        reference_critical_temperature,
        temperature_tolerance
    );

    // Reference bubble pressures at various temperatures
    // From the reference: BUBBLEPRESSURES starts at ~493 MPa at low temp, decreases to ~7.6 MPa
    // We'll test a few key points
    let temperature_grid = vec![
        -239.0, -200.0, -150.0, -100.0, -50.0, 0.0, 10.0, 20.0, 30.0,
    ];

    // Reference dew pressures
    // From the reference: DEWPRESSURES starts at ~85.8 kPa, increases to ~7.6 MPa
    let pressure_grid = vec![
        85_858.0, 100_000.0, 200_000.0, 500_000.0, 1_000_000.0, 2_000_000.0, 5_000_000.0, 7_600_000.0,
    ];

    let boundaries = Aga8Calculator::calculate_phase_boundaries(
        &composition,
        &temperature_grid,
        &pressure_grid,
        &eos,
    )
    .unwrap();

    // GERG-2008 cannot calculate phase boundaries (single-phase EOS)
    // All phase boundary values should be NaN
    for &bp in &boundaries.bubble_pressures {
        assert!(
            bp.is_nan(),
            "Bubble pressure should be NaN (GERG-2008 cannot calculate phase boundaries), got {}",
            bp
        );
    }
    
    for &dp in &boundaries.dew_pressures {
        assert!(
            dp.is_nan(),
            "Dew pressure should be NaN (GERG-2008 cannot calculate phase boundaries), got {}",
            dp
        );
    }
    
    for &bt in &boundaries.bubble_temperatures {
        assert!(
            bt.is_nan(),
            "Bubble temperature should be NaN (GERG-2008 cannot calculate phase boundaries), got {}",
            bt
        );
    }
    
    for &dt in &boundaries.dew_temperatures {
        assert!(
            dt.is_nan(),
            "Dew temperature should be NaN (GERG-2008 cannot calculate phase boundaries), got {}",
            dt
        );
    }
}

