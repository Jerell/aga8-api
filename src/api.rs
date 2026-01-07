use crate::models::ThermodynamicDataRequest;
use crate::service::ThermodynamicService;
use poem_openapi::{
    ApiResponse, OpenApi, OpenApiService,
    payload::{Json, PlainText},
};

/// Error response
#[derive(ApiResponse)]
pub enum ErrorResponse {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

/// API handler
pub struct Api;

#[OpenApi]
impl Api {
    /// Generate thermodynamic data and return as tab file
    #[oai(path = "/generate-tab-file", method = "post")]
    async fn generate_tab_file(
        &self,
        request: Json<ThermodynamicDataRequest>,
    ) -> Result<PlainText<String>, ErrorResponse> {
        let data = ThermodynamicService::generate_data(&request.0)
            .map_err(|e| ErrorResponse::BadRequest(PlainText(e.to_string())))?;

        let tab_content = ThermodynamicService::generate_tab_file(&data);
        Ok(PlainText(tab_content))
    }

    /// Generate thermodynamic data and return as JSON
    #[oai(path = "/generate-data", method = "post")]
    async fn generate_data(
        &self,
        request: Json<ThermodynamicDataRequest>,
    ) -> Result<Json<serde_json::Value>, ErrorResponse> {
        let data = ThermodynamicService::generate_data(&request.0)
            .map_err(|e| ErrorResponse::BadRequest(PlainText(e.to_string())))?;

        // Convert to JSON (simplified - you may want to create a proper serialization)
        let json = serde_json::json!({
            "composition": data.composition,
            "critical_point": data.critical_point,
            "phase_boundaries": data.phase_boundaries,
            "pressure_grid": data.pressure_grid,
            "temperature_grid": data.temperature_grid,
            "num_points": data.points.len()
        });

        Ok(Json(json))
    }

    /// Health check endpoint
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> PlainText<&'static str> {
        PlainText("OK")
    }
}

/// Create the OpenAPI service
pub fn create_api_service() -> OpenApiService<Api, ()> {
    OpenApiService::new(Api, "Thermodynamic Data API", "1.0")
        .description("API for generating thermodynamic data using AGA8 equation of state")
        .server("http://localhost:3000")
}
