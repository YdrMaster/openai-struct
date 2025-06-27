/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub MessageDeltaObjectDelta : The delta containing the fields that have changed on the Message.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaObjectDelta {
    /// The content of the message in array of text and/or images.
    #[serde(rename = "content")]
    pub content: Option<Vec<Value>>,
    /// The entity that produced the message. One of `user` or `assistant`.
    #[serde(rename = "role")]
    pub role: Option<String>,
}
