use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use reqwest::StatusCode;

use crate::{
    api::{
        cloudflare_ai::{CloudflareApi, CloudflareModel},
        google_gemini::GeminiApi,
        google_places::{GoogleGeocodeApiRequest, GoogleGeocodeApiResponse, GooglePlacesApi},
        google_vision::{GoogleVisionApi, GoogleVisionApiResponse, VisionFeatures},
        open_ai::{OpenAIApi, OpenAiModel},
    },
    models::{custom_error::CustomError, self, embedding_body_request::EmbeddintBodyRequest},
    repository::{local_storage::LocalStorage, prompt_provider},
    utils::{gps_utils::GpsUtils, image_utils::ImageUtils},
};

pub struct ExtApiUsecase {
    open_ai: Arc<OpenAIApi>,
    google_vision_api: Arc<GoogleVisionApi>,
    cloudflare_api: Arc<CloudflareApi>,
    gemini_api: Arc<GeminiApi>,
    google_places: Arc<GooglePlacesApi>,
    local_storage: Arc<LocalStorage>,
}

impl ExtApiUsecase {
    pub fn new(
        open_ai: Arc<OpenAIApi>,
        google_vision_api: Arc<GoogleVisionApi>,
        cloudflare_api: Arc<CloudflareApi>,
        gemini_api: Arc<GeminiApi>,
        google_places: Arc<GooglePlacesApi>,
        local_storage: Arc<LocalStorage>,
    ) -> Self {
        Self {
            open_ai,
            google_vision_api,
            cloudflare_api,
            gemini_api,
            google_places,
            local_storage,
        }
    }

    // Google Vision API 

    pub async fn vision(&self, base64_image: &str) -> Result<GoogleVisionApiResponse, CustomError> {
        self.google_vision_api
            .vision(
                base64_image,
                vec![
                    VisionFeatures::FaceDetection,
                    VisionFeatures::TextDetection,
                ],
            )
            .await
    }

    // Google Places API

    pub async fn google_geocoding(
        &self,
        location: GoogleGeocodeApiRequest,
    ) -> Result<GoogleGeocodeApiResponse, CustomError> {
        if !GpsUtils::is_valid_coordinate(location.lat, location.lng) {
            return Err(CustomError::InternalServerError(StatusCode::BAD_REQUEST));
        }
        self.google_places.geocoding(location).await
    }

    // OpenAI API

    pub async fn gpt_completion(&self, message: &str) -> Result<String, CustomError> {
        self.open_ai
            .completion(
                OpenAiModel::Gpt4Turbo,
                prompt_provider::Prompt::Poi,
                message,
            )
            .await
    }

    pub async fn gpt_visual(&self, f: TempFile) -> Result<String, CustomError> {
        let path = self.local_storage.persist(f)?;
        match ImageUtils::to_base64(&path) {
            Ok(base64_image) => {
                self.open_ai
                    .visual(
                        OpenAiModel::Gpt4Visual,
                        prompt_provider::Prompt::Poi,
                        &base64_image,
                    )
                    .await
            }
            Err(e) => Err(e),
        }
    }

    // Gemini API | Google

    pub async fn gemini_completion(&self, message: &str) -> Result<String, CustomError> {
        self.gemini_api
            .completion(prompt_provider::Prompt::Poi, message)
            .await
    }

    pub async fn gemini_visual(&self, f: TempFile) -> Result<String, CustomError> {
        let path = self.local_storage.persist(f)?;
        match ImageUtils::to_base64(&path) {
            Ok(base64_image) => {
                self.gemini_api
                    .visual(prompt_provider::Prompt::Poi, &base64_image)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    // Cloudflare API

    pub async fn llama_completion(&self, message: &str) -> Result<String, CustomError> {
        self.cloudflare_api
            .completion(
                CloudflareModel::Llama27b,
                prompt_provider::Prompt::Poi,
                message,
            )
            .await
    }

    pub async fn embedding(&self, body: &EmbeddintBodyRequest) -> Result<Vec<Vec<f64>>, CustomError> {
        self.cloudflare_api
            .embedding(body)
            .await
    }

}
