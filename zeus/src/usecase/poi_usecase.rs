use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;

use crate::{
    api::{
        google_gemini::GeminiApi,
        google_places::{GoogleGeocodeApiRequest, GooglePlacesApi},
        google_vision::{GoogleVisionApi, VisionFeatures},
        open_ai::{OpenAIApi, OpenAiModel},
    },
    models::custom_error::CustomError,
    repository::prompt_provider,
    utils::image_utils::{self, ImageUtils},
};

pub struct PoiUsecase {
    google_vision_api: Arc<GoogleVisionApi>,
    gemini_api: Arc<GeminiApi>,
    open_ai: Arc<OpenAIApi>,
    google_places: Arc<GooglePlacesApi>,
}

impl PoiUsecase {
    pub fn new(
        google_vision_api: Arc<GoogleVisionApi>,
        gemini_api: Arc<GeminiApi>,
        open_ai: Arc<OpenAIApi>,
        google_places: Arc<GooglePlacesApi>,
    ) -> Self {
        Self {
            google_vision_api,
            gemini_api,
            open_ai,
            google_places,
        }
    }

    pub async fn from_image(
        &self,
        location: GoogleGeocodeApiRequest,
        f: TempFile,
    ) -> Result<String, CustomError> {
        let path = f.file.path().to_str().unwrap();
        match ImageUtils::to_base64(&path) {
            Ok(base64_image) => {
                //Vision
                let vision = self
                    .google_vision_api
                    .vision(
                        base64_image.as_str(),
                        vec![VisionFeatures::DocumentTextDetection],
                    )
                    .await?;
                let vision_compact = vision.text();
                log::debug!("\n\tPOI vision: {}", vision_compact);

                //Geocodding
                let geocoding = self.google_places.geocoding(location).await?;
                let geocoding_compact = serde_json::to_string(&geocoding.unique_addresses())?;

                log::debug!("\n\tPOI geocodding: {}", geocoding_compact);

                //Visual - Gemini
                let gemini_visual = self
                    .gemini_api
                    .visual(prompt_provider::Prompt::PoiVisual, &base64_image)
                    .await?;

                log::debug!("\n\tPOI visual Gemini: {}", gemini_visual);

                //Visual - GPT
                let gpt_visual = self
                    .open_ai
                    .visual(
                        OpenAiModel::Gpt4Visual,
                        prompt_provider::Prompt::PoiVisual,
                        &base64_image,
                    )
                    .await?;

                log::debug!("\n\tPOI visual GPT: {}", gpt_visual);

                //Summary
                let request = format!(
                    "POI description from person one: {}. Description from another person: {}. Potential Address: {}. OCR results: {}. Potentiall GPS lat = {} ; lng = {}",
                    gemini_visual, gpt_visual, geocoding_compact, vision_compact, location.lat, location.lng,
                );

                let poi_gpt = self
                    .open_ai
                    .completion(
                        OpenAiModel::Gpt4Turbo,
                        prompt_provider::Prompt::Poi,
                        request.as_str(),
                    )
                    .await?;

                log::debug!("\n\tPOI GPT summary: {}", poi_gpt);

                let poi_gemini = self
                    .gemini_api
                    .completion(prompt_provider::Prompt::Poi, request.as_str())
                    .await?;
                log::debug!("\n\tPOI Gemini summary: {}", poi_gemini);
                Ok(format!("GPT: {}\nGemini: {}", poi_gpt, poi_gemini,))
            }
            Err(e) => Err(e),
        }
    }
}
