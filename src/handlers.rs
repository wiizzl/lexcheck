use crate::models::{UserData, ValidationResponse};
use crate::services::{ComplianceError, check_compliance};
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn validate_handler(Json(payload): Json<UserData>) -> impl IntoResponse {
    match check_compliance(&payload) {
        Ok(msg) => {
            let resp = ValidationResponse {
                status: "APPROVED".to_string(),
                message: msg,
            };
            (StatusCode::OK, Json(resp))
        }
        Err(ComplianceError::Underage) => {
            let resp = ValidationResponse {
                status: "REJECTED".to_string(),
                message: "User is underage.".to_string(),
            };
            (StatusCode::FORBIDDEN, Json(resp))
        }
        Err(ComplianceError::PendingDocuments) => {
            let resp = ValidationResponse {
                status: "PENDING".to_string(),
                message: "User has pending documents.".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}
