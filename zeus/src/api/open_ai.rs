use serde::{Deserialize, Serialize};

use crate::{
    models::custom_error::CustomError,
    repository::{prompt_provider::Prompt, secrets::Secrets},
};

pub struct OpenAIApi {
    client: reqwest::Client,
}

impl OpenAIApi {
    const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";
    const MAX_TOKENS: u32 = 4096;

    pub fn new(secrets: &Secrets) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", secrets.open_ai_api_key))
                .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");
        Self { client }
    }

    pub async fn completion(&self, model: OpenAiModel, prompt: Prompt, message: &str) -> Result<String, CustomError> {
        let messages = vec![
            Role::System.new(MessageContent::SimpleText(prompt.prompt())),
            Role::User.new(MessageContent::SimpleText(message.to_string())),
        ];

        let payload = Payload {
            model: model.name(),
            messages,
            temperature: 1,
            max_tokens: Some(Self::MAX_TOKENS),
        };

        let response: reqwest::Response = self
            .client
            .post(Self::API_URL)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion: ChatCompletion = response.json().await?;
            chat_completion.assistant_response_text()
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }

    pub async fn visual(&self, model: OpenAiModel, prompt: Prompt, base64_image: &str) -> Result<String, CustomError> {
        let payload = Payload {
            model: model.name(),
            messages: vec![Role::User.new(MessageContent::DetailedContent(vec![
                ContentType::Text {
                    text: prompt.prompt(),
                },
                ContentType::ImageUrl {
                    image_url: ImageUrl::base64(base64_image),
                },
            ]))],
            temperature: 1,
            max_tokens: Some(Self::MAX_TOKENS),
        };

        let response = self
            .client
            .post(Self::API_URL)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_completion: ChatCompletion = response.json().await?;
            chat_completion.assistant_response_text()
        } else {
            let code = response.status().as_u16();
            if let Ok(text) = response.text().await {
                log::error!("Error response: {}", text);
            }
            Err(CustomError::NonSuccessfulResponse(code))
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

impl ChatCompletion {
    fn assistant_response_text(&self) -> Result<String, CustomError> {
        if let Some(assitant_choise) = self.choices.iter().find(|choice| {
            if choice.message.role == Role::Assistant.role() {
                true
            } else {
                false
            }
        }) {
            let content = &assitant_choise.message.content;
            match content {
                MessageContent::SimpleText(text) => Ok(text.to_owned()),
                _ => Err(CustomError::NoContentFromAssistant),
            }
        } else {
            Err(CustomError::NoContentFromAssistant)
        }
    }
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: Option<String>,
    index: i64,
}

// Enum to represent different types of content
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ContentType {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

// Struct to hold the image URL
#[derive(Debug, Serialize, Deserialize)]
struct ImageUrl {
    url: String,
}

impl ImageUrl {
    fn base64(base64: &str) -> Self {
        Self {
            url: format!("data:image/jpeg;base64,{}", base64),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum MessageContent {
    SimpleText(String),
    DetailedContent(Vec<ContentType>),
}

// Struct for the message
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: MessageContent,
}

// Struct for the payload
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    model: String,
    messages: Vec<Message>,
    temperature: i64,
    max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    fn role(&self) -> String {
        match self {
            Role::System => "system".to_string(),
            Role::User => "user".to_string(),
            Role::Assistant => "assistant".to_string(),
        }
    }

    fn new(&self, content: MessageContent) -> Message {
        Message {
            role: self.role(),
            content: content,
        }
    }
}

pub enum OpenAiModel {
    Gpt4Turbo,
    Gpt4Visual,
}

impl OpenAiModel {
    fn name(&self) -> String {
        match self {
            OpenAiModel::Gpt4Turbo => "gpt-4-1106-preview".to_string(),
            OpenAiModel::Gpt4Visual => "gpt-4-vision-preview".to_string(),
        }
    }
}
