use aga8_api::models::{Composition, CriticalPoint, PhaseBoundaries, ThermodynamicPoint};
use aga8_api::tab_file::TabFileFormatter;

fn create_test_composition() -> Composition {
    Composition {
        components: vec!["CO2".to_string(), "H2".to_string()],
        mole_fractions: vec![0.99, 0.01],
    }
}

fn create_test_critical_point() -> CriticalPoint {
    CriticalPoint {
        pressure: 7_681_406.0,
        temperature: 30.7845310,
    }
}

fn create_test_phase_boundaries() -> PhaseBoundaries {
    PhaseBoundaries {
        bubble_pressures: vec![493_595_079.0, 482_583_781.0],
        bubble_temperatures: vec![-239.04009, -238.87176],
        dew_pressures: vec![85_858.5859, 153_668.979],
        dew_temperatures: vec![-89.704443, -80.393831],
    }
}

fn create_test_point() -> ThermodynamicPoint {
    ThermodynamicPoint {
        pressure: 85_858.5859,
        temperature: -20.0,
        gas_density: 1.79081970,
        liquid_density: 1008.35187,
        d_rho_gas_dp: 2.10077e-5,
        d_rho_liq_dp: 4.16220e-6,
        d_rho_gas_dt: -0.00722354,
        d_rho_liq_dt: -4.7006394,
        rs: 1.0,
        gas_viscosity: 1.29447e-5,
        liquid_viscosity: 0.000148339,
        gas_cp: 806.095759,
        liquid_cp: 1865.59489,
        gas_enthalpy: -38093.301,
        liquid_enthalpy: -349375.58,
        gas_thermal_conductivity: 0.014448240,
        liquid_thermal_conductivity: 0.132125757,
        surface_tension: 0.009313488,
        gas_entropy: -97.595648,
        liquid_entropy: -1890.9995,
    }
}

/// Test tab file header formatting
#[test]
fn test_tab_file_header() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![85_858.5859, 171_717.172];
    let temperature_grid = vec![-20.0, -19.444444];
    let points = vec![create_test_point()];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    // Check header contains key elements
    assert!(
        output.contains("PVTTABLE LABEL"),
        "Should contain PVTTABLE LABEL"
    );
    assert!(
        output.contains("PHASE = TWO"),
        "Should contain phase specification"
    );
    assert!(output.contains("COMPONENTS ="), "Should contain components");
    assert!(output.contains("CO2"), "Should contain CO2 component");
    assert!(output.contains("H2"), "Should contain H2 component");
    assert!(output.contains("MOLES ="), "Should contain mole fractions");
    assert!(
        output.contains("0.990000000"),
        "Should contain first mole fraction"
    );
    assert!(
        output.contains("0.010000000"),
        "Should contain second mole fraction"
    );
}

/// Test tab file critical point formatting
#[test]
fn test_tab_file_critical_point() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![100_000.0];
    let temperature_grid = vec![20.0];
    let points = vec![];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    assert!(
        output.contains("CRITICALPRESSURE"),
        "Should contain critical pressure"
    );
    assert!(
        output.contains("CRITICALTEMPERATURE"),
        "Should contain critical temperature"
    );
    assert!(
        output.contains("7681406.00"),
        "Should contain critical pressure value"
    );
    assert!(
        output.contains("30.7845310"),
        "Should contain critical temperature value"
    );
}

/// Test tab file phase boundaries formatting
#[test]
fn test_tab_file_phase_boundaries() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![100_000.0];
    let temperature_grid = vec![20.0];
    let points = vec![];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    assert!(
        output.contains("BUBBLEPRESSURES"),
        "Should contain bubble pressures"
    );
    assert!(
        output.contains("BUBBLETEMPERATURES"),
        "Should contain bubble temperatures"
    );
    assert!(
        output.contains("DEWPRESSURES"),
        "Should contain dew pressures"
    );
    assert!(
        output.contains("DEWTEMPERATURES"),
        "Should contain dew temperatures"
    );
}

/// Test tab file data point formatting
#[test]
fn test_tab_file_data_point() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![85_858.5859];
    let temperature_grid = vec![-20.0];
    let point = create_test_point();
    let points = vec![point];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    assert!(
        output.contains("PVTTABLE POINT"),
        "Should contain PVTTABLE POINT"
    );
    assert!(
        output.contains("COLUMNS ="),
        "Should contain column definitions"
    );

    // Check that the point data is formatted correctly
    // The exact format should match the expected tab file format
    let point_line = output
        .lines()
        .find(|line| line.contains("PVTTABLE POINT"))
        .expect("Should find PVTTABLE POINT line");

    // Verify the point contains the expected values
    // Pressure is formatted with {:.2}, so 85858.5859 becomes 85858.59
    assert!(
        point_line.contains("85858.59") || point_line.contains("85858.5859"),
        "Should contain pressure value"
    );
    assert!(
        point_line.contains("-20.000000"),
        "Should contain temperature value"
    );
}

/// Test tab file column definitions
#[test]
fn test_tab_file_columns() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![100_000.0];
    let temperature_grid = vec![20.0];
    let points = vec![];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    assert!(
        output.contains("COLUMNS ="),
        "Should contain column definitions"
    );
    // Check for key column names
    assert!(output.contains("PT"), "Should contain PT column");
    assert!(output.contains("TM"), "Should contain TM column");
    assert!(output.contains("ROG"), "Should contain ROG column");
    assert!(output.contains("ROHL"), "Should contain ROHL column");
    assert!(output.contains("VISG"), "Should contain VISG column");
    assert!(output.contains("VISHL"), "Should contain VISHL column");
}

/// Test tab file data point serialization format
#[test]
fn test_data_point_serialization() {
    let point = create_test_point();
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![point.pressure];
    let temperature_grid = vec![point.temperature];
    let points = vec![point];

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    // Find the PVTTABLE POINT line
    let point_line = output
        .lines()
        .find(|line| line.contains("PVTTABLE POINT"))
        .expect("Should find PVTTABLE POINT line");

    // Verify the line has the correct structure
    // Should start with "PVTTABLE POINT = ("
    assert!(
        point_line.starts_with("PVTTABLE POINT = ("),
        "Point line should start with correct prefix"
    );

    // Should end with " )"
    assert!(
        point_line.trim_end().ends_with(")"),
        "Point line should end with closing parenthesis"
    );

    // Count the number of values (should be 20 based on COLUMNS definition)
    let values: Vec<&str> = point_line
        .split('(')
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .split(',')
        .collect();

    // Should have 20 values for the 20 columns
    assert_eq!(
        values.len(),
        20,
        "Should have 20 values matching the 20 columns"
    );
}

/// Test tab file with multiple data points
#[test]
fn test_tab_file_multiple_points() {
    let composition = create_test_composition();
    let critical = create_test_critical_point();
    let boundaries = create_test_phase_boundaries();
    let pressure_grid = vec![85_858.5859, 171_717.172];
    let temperature_grid = vec![-20.0, -19.444444];

    let mut points = Vec::new();
    for &pressure in &pressure_grid {
        for &temperature in &temperature_grid {
            points.push(ThermodynamicPoint {
                pressure,
                temperature,
                gas_density: 1.79,
                liquid_density: 1008.35,
                d_rho_gas_dp: 2.1e-5,
                d_rho_liq_dp: 4.16e-6,
                d_rho_gas_dt: -0.007,
                d_rho_liq_dt: -4.7,
                rs: 1.0,
                gas_viscosity: 1.29e-5,
                liquid_viscosity: 0.000148,
                gas_cp: 806.0,
                liquid_cp: 1865.0,
                gas_enthalpy: -38093.0,
                liquid_enthalpy: -349375.0,
                gas_thermal_conductivity: 0.0144,
                liquid_thermal_conductivity: 0.132,
                surface_tension: 0.0093,
                gas_entropy: -97.6,
                liquid_entropy: -1891.0,
            });
        }
    }

    let output = TabFileFormatter::format(
        &composition,
        &critical,
        &boundaries,
        &pressure_grid,
        &temperature_grid,
        &points,
    );

    // Count PVTTABLE POINT lines
    let point_count = output
        .lines()
        .filter(|l| l.contains("PVTTABLE POINT"))
        .count();
    assert_eq!(
        point_count,
        points.len(),
        "Should have one PVTTABLE POINT line per data point"
    );
}
