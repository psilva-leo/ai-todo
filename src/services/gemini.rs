use crate::error::AppError;
use base64::{Engine as _, engine::general_purpose};
use serde_json::json;
use std::env;

#[derive(Clone)]
pub struct GeminiService {
    api_key: String,
    client: reqwest::Client,
}

impl GeminiService {
    pub fn new() -> Result<Self, AppError> {
        let api_key = env::var("GOOGLE_API_KEY")
            .map_err(|_| AppError::Internal("GOOGLE_API_KEY not set".to_string()))?;
        Ok(Self {
            api_key,
            client: reqwest::Client::new(),
        })
    }

    pub async fn suggest_tasks(
        &self,
        audio_data: Vec<u8>,
        mime_type: &str,
    ) -> Result<Vec<crate::models::todo::SuggestedTodo>, AppError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.api_key
        );

        let base64_audio = general_purpose::STANDARD.encode(audio_data);

        let payload = json!({
            "contents": [{
                "parts": [
                    {"text": "Extract a list of actionable tasks from this audio. Return ONLY a JSON array of objects. Each object MUST have 'title' (string), 'description' (string or null), and 'priority' (string: 'Low', 'Medium', or 'High'). No other text, no markdown code blocks."},
                    {
                        "inline_data": {
                            "mime_type": mime_type,
                            "data": base64_audio
                        }
                    }
                ]
            }],
            "generationConfig": {
                "response_mime_type": "application/json"
            }
        });

        let resp = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to call Gemini API: {e}")))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!("Gemini API error: {err_text}")));
        }

        let gemini_resp: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse Gemini response: {e}")))?;

        let text = gemini_resp["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| {
                AppError::Internal("Unexpected Gemini response structure".to_string())
            })?;

        let tasks: Vec<crate::models::todo::SuggestedTodo> =
            serde_json::from_str(text).map_err(|e| {
                AppError::Internal(format!(
                    "Failed to parse tasks from Gemini: {e}. Text was: {text}"
                ))
            })?;

        Ok(tasks)
    }
}
