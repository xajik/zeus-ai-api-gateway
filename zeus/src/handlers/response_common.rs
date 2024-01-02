use actix_web::HttpResponse;
use serde_json::json;

use crate::models::custom_error::CustomError;

pub fn create_response<T: serde::Serialize>(result: Result<T, CustomError>) -> HttpResponse {
    match result {
        Ok(response) => HttpResponse::Ok().json(json!({"status": "success", "message": response})),
        Err(e) => {
            log::error!("\n Generating error response: \n {:?} \n", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Something went wrong"}))
        },
    }
}