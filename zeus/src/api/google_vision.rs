use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{models::custom_error::CustomError, repository::secrets::Secrets};

pub struct GoogleVisionApi {
    client: reqwest::Client,
    key: (String, String),
}

impl GoogleVisionApi {
    const API_URL: &'static str = "https://vision.googleapis.com/v1/images:annotate";

    pub fn new(secrets: &Secrets) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        let key = ("key".to_string(), secrets.google_vision_api_key.clone());
        Self { client, key }
    }

    pub async fn vision(
        &self,
        base64_image: &str,
        features: Vec<VisionFeatures>,
    ) -> Result<GoogleVisionApiResponse, CustomError> {
        let features = features
            .iter()
            .map(|f| Feature { type_: f.feature() })
            .collect();

        let body = VisionRequest {
            requests: vec![VisionRequestItem {
                image: ImageData {
                    content: base64_image.to_string(),
                },
                features: features,
                image_context: None,
            }],
        };

        let res = self
            .client
            .post(Self::API_URL)
            .query(&[self.key.clone()])
            .json(&body)
            .send()
            .await?;

        if res.status().is_success() {
            let response: GoogleVisionApiResponse = res.json().await?;
            Ok(response)
        } else {
            let code = res.status().as_u16();
            if let Ok(text) = res.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VisionRequest {
    requests: Vec<VisionRequestItem>,
}

#[derive(Serialize, Deserialize)]
struct VisionRequestItem {
    image: ImageData,
    features: Vec<Feature>,
    #[serde(rename = "imageContext")]
    image_context: Option<ImageContext>,
}

#[derive(Serialize, Deserialize)]
struct ImageData {
    content: String, // base64 encoded image
}

#[derive(Serialize, Deserialize)]
struct Feature {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize)]
struct ImageContext {
    #[serde(rename = "languageHints")]
    language_hints: Vec<String>,
}

pub enum VisionFeatures {
    DocumentTextDetection,
    TextDetection,
    FaceDetection,
}

impl VisionFeatures {
    fn feature(&self) -> String {
        match self {
            Self::DocumentTextDetection => "DOCUMENT_TEXT_DETECTION".to_string(),
            Self::TextDetection => "TEXT_DETECTION".to_string(),
            Self::FaceDetection => "FACE_DETECTION".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleVisionApiResponse {
    responses: Vec<ResponseItem>,
}

impl GoogleVisionApiResponse {
    
    pub fn text(&self) -> String {
        self.responses
            .iter()
            .map(|r| r.text())
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn faces(&self) -> Vec<GoogleVisionFaceAnnotation> {
        self.responses
            .iter()
            .filter_map(|r| r.face_annotations.clone())
            .flatten()
            .collect()
    }

    pub fn text_annotations(&self) -> Vec<GoogleVisionTextAnnotation> {
        self.responses
            .iter()
            .filter_map(|r| r.text_annotations.clone())
            .flatten()
            .collect()
    }



}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseItem {
    #[serde(rename = "faceAnnotations")]
    face_annotations: Option<Vec<GoogleVisionFaceAnnotation>>,
    #[serde(rename = "textAnnotations")]
    text_annotations: Option<Vec<GoogleVisionTextAnnotation>>,
    #[serde(rename = "fullTextAnnotation")]
    full_text_annotation: Option<FullTextAnnotation>,
}

impl ResponseItem {
    pub fn text(&self) -> String {
        if let Some(full_text_annotation) = &self.full_text_annotation {
            full_text_annotation.text.clone()
        } else {
            "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleVisionFaceAnnotation {
    #[serde(rename = "boundingPoly")]
    bounding_poly: BoundingPoly,

    #[serde(rename = "fdBoundingPoly")]
    fd_bounding_poly: BoundingPoly,

    landmarks: Vec<Landmark>,

    #[serde(rename = "rollAngle")]
    roll_angle: f32,

    #[serde(rename = "panAngle")]
    pan_angle: f32,

    #[serde(rename = "tiltAngle")]
    tilt_angle: f32,

    #[serde(rename = "detectionConfidence")]
    detection_confidence: f32,

    #[serde(rename = "landmarkingConfidence")]
    landmarking_confidence: f32,

    #[serde(rename = "joyLikelihood")]
    joy_likelihood: String,

    #[serde(rename = "sorrowLikelihood")]
    sorrow_likelihood: String,

    #[serde(rename = "angerLikelihood")]
    anger_likelihood: String,

    #[serde(rename = "surpriseLikelihood")]
    surprise_likelihood: String,

    #[serde(rename = "underExposedLikelihood")]
    under_exposed_likelihood: String,

    #[serde(rename = "blurredLikelihood")]
    blurred_likelihood: String,

    #[serde(rename = "headwearLikelihood")]
    headwear_likelihood: String,
}

// Definitions for BoundingPoly and Landmark structs should also be provided.

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BoundingPoly {
    vertices: Vec<Vertex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Vertex {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Landmark {
    #[serde(rename = "type")]
    type_: String,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleVisionTextAnnotation {
    locale: Option<String>,
    description: String,
    #[serde(rename = "boundingPoly")]
    bounding_poly: BoundingPoly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FullTextAnnotation {
    pages: Vec<Page>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Page {
    property: HashMap<String, Vec<Language>>,
    width: i32,
    height: i32,
    blocks: Vec<Block>,
    confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Language {
    #[serde(rename = "languageCode")]
    language_code: String,
    confidence: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingPoly,
    paragraphs: Vec<Paragraph>,
    #[serde(rename = "blockType")]
    block_type: String,
    confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Paragraph {
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingPoly,
    words: Vec<Word>,
    confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Word {
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingPoly,
    symbols: Vec<Symbol>,
    confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Symbol {
    property: Option<HashMap<String, DetectedBreak>>,
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingPoly,
    text: String,
    confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DetectedBreak {
    #[serde(rename = "type")]
    type_: String,
}
