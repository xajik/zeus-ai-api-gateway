use crate::{
    models::{custom_error::CustomError, embedding_body_request::EmbeddintBodyRequest},
    repository::{prompt_provider::Prompt, secrets::Secrets},
};
use serde::{Deserialize, Serialize};

pub struct CloudflareApi {
    client: reqwest::Client,
    account: String,
}

impl CloudflareApi {
    const API_URL: &'static str =
        "https://api.cloudflare.com/client/v4/accounts/{account}/ai/run/@cf/{model}";

    pub fn new(secrets: &Secrets) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!(
                "Bearer {}",
                secrets.cloudflare_api_key
            ))
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            account: secrets.cloudflare_account.clone(),
        }
    }

    pub async fn completion(
        &self,
        model: CloudflareModel,
        prompt: Prompt,
        message: &str,
    ) -> Result<String, CustomError> {
        let messages = vec![
            Message {
                role: CloudflareRole::System.name(),
                content: prompt.prompt(),
            },
            Message {
                role: CloudflareRole::User.name(),
                content: message.to_string(),
            },
        ];

        let request = RequestBody { messages };

        log::info!("Cloudflare payload: {:?}", serde_json::to_string(&request));

        let url = Self::API_URL
            .replace("{account}", self.account.as_str())
            .replace("{model}", model.name());
        let response: reqwest::Response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let chat_completion: CloudflareResponseBody = response.json().await?;
            log::info!("Chat completion: {:?}", chat_completion);
            Ok(chat_completion.result.response)
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }

    pub async fn embedding(
        &self,
        body: &EmbeddintBodyRequest,
    ) -> Result<Vec<Vec<f64>>, CustomError> {
        let model = CloudflareModel::BgeBaseEn;

        let url = Self::API_URL
            .replace("{account}", self.account.as_str())
            .replace("{model}", model.name());

        let response: reqwest::Response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let emdebbings: EmbeddingApiResponse = response.json().await?;
            Ok(emdebbings.result.data)
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }
}

pub enum CloudflareModel {
    Llama27b,
    BgeBaseEn,
}

impl CloudflareModel {
    fn name(&self) -> &str {
        match self {
            CloudflareModel::Llama27b => "meta/llama-2-7b-chat-int8",
            CloudflareModel::BgeBaseEn => "baai/bge-base-en-v1.5",
        }
    }
}

enum CloudflareRole {
    User,
    System,
}

impl CloudflareRole {
    fn name(&self) -> String {
        match self {
            CloudflareRole::User => "user".to_string(),
            CloudflareRole::System => "system".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CloudflareResponseBody {
    result: CompletionResultData,
    success: bool,
    errors: Vec<String>,
    messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionResultData {
    response: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct EmbeddingApiResponse {
    result: EmbeddingResultData,
    success: bool,
    errors: Vec<String>,
    messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingResultData {
    shape: Vec<i32>,
    data: Vec<Vec<f64>>,
}

