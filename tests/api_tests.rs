use aga8_api::models::{Composition, EnthalpyRange, PressureRange, ThermodynamicDataRequest};
use aga8_api::service::ThermodynamicService;

fn create_test_request() -> ThermodynamicDataRequest {
    ThermodynamicDataRequest {
        composition: Composition {
            components: vec!["CO2".to_string(), "H2".to_string()],
            mole_fractions: vec![0.99, 0.01],
        },
        pressure_range: PressureRange {
            min: 100_000.0, // Minimum valid pressure
            max: 850_000.0,
            points: 5,
        },
        enthalpy_range: EnthalpyRange {
            min: -100_000.0,
            max: 100_000.0,
            points: 10,
        },
        equation_of_state: "gerg2008".to_string(), // Default to GERG-2008
    }
}

/// Test complete data generation workflow
#[test]
fn test_generate_data() {
    let request = create_test_request();

    let data = ThermodynamicService::generate_data(&request).unwrap();

    // Validate composition
    assert_eq!(data.composition.components.len(), 2);
    assert_eq!(data.composition.mole_fractions.len(), 2);

    // Validate grids
    assert_eq!(data.pressure_grid.len(), 5);
    assert_eq!(data.temperature_grid.len(), 10);

    // Validate critical point - should be calculated (not NaN)
    assert!(!data.critical_point.pressure.is_nan());
    assert!(!data.critical_point.temperature.is_nan());
    assert!(data.critical_point.pressure > 0.0);
    assert!(data.critical_point.temperature > -100.0 && data.critical_point.temperature < 150.0);

    // Validate phase boundaries
    assert_eq!(
        data.phase_boundaries.bubble_pressures.len(),
        data.temperature_grid.len()
    );
    assert_eq!(
        data.phase_boundaries.bubble_temperatures.len(),
        data.pressure_grid.len()
    );

    // Validate points
    assert_eq!(data.points.len(), 5 * 10); // pressure_grid.len() * temperature_grid.len()
}

/// Test tab file generation
#[test]
fn test_generate_tab_file() {
    let request = create_test_request();

    let data = ThermodynamicService::generate_data(&request).unwrap();
    let tab_content = ThermodynamicService::generate_tab_file(&data);

    // Validate tab file structure
    assert!(tab_content.contains("PVTTABLE LABEL"));
    assert!(tab_content.contains("COMPONENTS ="));
    assert!(tab_content.contains("CRITICALPRESSURE"));
    assert!(tab_content.contains("CRITICALTEMPERATURE"));
    assert!(tab_content.contains("COLUMNS ="));
    assert!(tab_content.contains("PVTTABLE POINT"));

    // Count data points
    let point_count = tab_content
        .lines()
        .filter(|l| l.contains("PVTTABLE POINT"))
        .count();
    assert_eq!(point_count, data.points.len());
}

/// Test request validation
#[test]
fn test_request_validation() {
    let mut request = create_test_request();

    // Valid request should work
    assert!(ThermodynamicService::generate_data(&request).is_ok());

    // Invalid composition (wrong sum)
    request.composition.mole_fractions = vec![0.99, 0.02];
    assert!(ThermodynamicService::generate_data(&request).is_err());

    // Fix composition
    request.composition.mole_fractions = vec![0.99, 0.01];

    // Invalid composition (mismatched lengths)
    request.composition.components = vec!["CO2".to_string()];
    assert!(ThermodynamicService::generate_data(&request).is_err());
}

/// Test with different grid sizes
#[test]
fn test_different_grid_sizes() {
    let mut request = create_test_request();

    // Small grid
    request.pressure_range.points = 2;
    request.enthalpy_range.points = 2;
    let data = ThermodynamicService::generate_data(&request).unwrap();
    assert_eq!(data.points.len(), 4);

    // Single point grids
    request.pressure_range.points = 1;
    request.enthalpy_range.points = 1;
    let data = ThermodynamicService::generate_data(&request).unwrap();
    assert_eq!(data.points.len(), 1);
}
