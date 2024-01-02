use std::fs::{self};

use actix_multipart::form::tempfile;
use chrono::{Utc, DateTime};

use crate::models::custom_error::CustomError;

pub struct LocalStorage;

impl LocalStorage {
    const UPLOAD_FOLDER: &'static str = "./target/cache/uploads";

    pub fn new() -> Self {
        fs::create_dir_all(Self::UPLOAD_FOLDER)
            .expect(format!("Unable to create upload folder: {}", Self::UPLOAD_FOLDER).as_str());
        Self {}
    }

    pub fn persist(&self, temp_file: tempfile::TempFile) -> Result<String, CustomError> {
        if let Some(file_name) = temp_file.file_name {
            let file_name = format!(
                "{}_{}",
                DateTime::<Utc>::from(Utc::now()).timestamp(),
                file_name
            );
            let path = format!("{}/{}", self.folder(), file_name);
            log::info!("saving to {path}", path = path);
            temp_file.file.persist(&path)
            .map_err(|e| CustomError::File(format!("Error persisting file: {}", e)))
            .map(|_| path)
        } else {
            Err(CustomError::File(format!("Error persisting file")))
        }
    }

    pub fn folder(&self) -> &'static str {
        Self::UPLOAD_FOLDER
    }
}
