use base64::{engine::general_purpose, Engine};

use crate::models::custom_error::CustomError;

pub struct ImageUtils {}

impl ImageUtils {

    pub fn to_base64(image_path: &str) -> Result<String, CustomError> {
        match std::fs::read(image_path) {
            Ok(image_data) => return Ok(general_purpose::STANDARD.encode(&image_data)),
            Err(e) => return Err(CustomError::IoError(e)),
        }
    }
}
