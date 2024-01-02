use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddintBodyRequest {
    text: Vec<String>,
}