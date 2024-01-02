use serde::{Deserialize, Serialize};

use crate::{
    models::custom_error::CustomError,
    repository::{prompt_provider::Prompt, secrets::Secrets},
};

pub struct GeminiApi {
    client: reqwest::Client,
    key: (String, String),
}

impl GeminiApi {
    const API_URL: &'static str =
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent";

    pub fn new(secrets: &Secrets) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        let key = ("key".to_string(), secrets.google_ai_studio_api_key.clone());
        Self { client, key }
    }

    pub async fn completion(&self, prompt: Prompt, message: &str) -> Result<String, CustomError> {
        let model = GeminiModel::Text;
        let role = GeminiRole::User;
        let model_message = format!("{} . \n {}", prompt.prompt(), message);
        let request = GeminiRequest {
            contents: vec![Content {
                role: role.to_string(),
                parts: vec![Part::TextPart {
                    text: model_message.to_string(),
                }],
            }],
        };
        let url = Self::API_URL.replace("{model}", &model.to_string());
        let response = self
            .client
            .post(&url)
            .query(&[self.key.clone()])
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion: Vec<GeminiResponse> = response.json().await?;
            Ok(chat_completion.iter()
            .flat_map(|completion| completion.combine_text_parts())
            .collect::<Vec<String>>()
            .join(" "))
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }

    pub async fn visual(&self, prompt: Prompt, base64_image: &str) -> Result<String, CustomError> {
        let model = GeminiModel::Vision;
        let role = GeminiRole::User;
        let request = GeminiRequest {
            contents: vec![Content {
                role: role.to_string(),
                parts: vec![
                    Part::TextPart {
                        text: prompt.prompt(),
                    },
                    Part::DataPart {
                    inline_data: InlineData {
                        mime_type: "image/jpeg".to_string(),
                        data: base64_image.to_string(),
                    },
                }],
            }],
        };
        let url = Self::API_URL.replace("{model}", &model.to_string());
        let response = reqwest::Client::new()
            .post(&url)
            .query(&[self.key.clone()])
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let text_text = &response.text().await?;
            log::debug!("text_text: {}", text_text);
            let chat_completion: Vec<GeminiResponse> = serde_json::from_str(text_text)?;
            Ok(chat_completion.iter()
            .flat_map(|completion| completion.combine_text_parts())
            .collect::<Vec<String>>()
            .join(" "))
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    } 
}

enum GeminiModel {
    Text,
    Vision,
}

impl GeminiModel {
    pub fn to_string(&self) -> String {
        match self {
            GeminiModel::Text => "gemini-pro",
            GeminiModel::Vision => "gemini-pro-vision",
        }
        .to_string()
    }
}

enum GeminiRole {
    Model,
    User,
}

impl GeminiRole {
    pub fn to_string(&self) -> String {
        match self {
            GeminiRole::Model => "model",
            GeminiRole::User => "user",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Part {
    TextPart { text: String },
    DataPart { inline_data: InlineData },
}

#[derive(Serialize, Deserialize, Debug)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiRootResponse {
    responses: Vec<GeminiResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    #[serde(rename = "promptFeedback")]
    prompt_feedback: Option<PromptFeedback>,
}

impl GeminiResponse {
    fn combine_text_parts(&self) -> Option<String> {
        let text = self
            .candidates
            .iter()
            .flat_map(|candidate| &candidate.content.parts)
            .filter_map(|part| match part {
                Part::TextPart { text } => Some(text.clone()),
                Part::DataPart { .. } => None,
            })
            .collect::<Vec<String>>()
            .join(" ");
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    content: ContentData,
    #[serde(rename = "finishReason")]
    finish_reason: String,
    index: i32,
    #[serde(rename = "safetyRatings")]
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentData {
    parts: Vec<Part>,
    role: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SafetyRating {
    category: String,
    probability: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    safety_ratings: Vec<SafetyRating>,
}
