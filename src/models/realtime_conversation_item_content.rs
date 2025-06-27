/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimeConversationItemContent {
    /// Base64-encoded audio bytes, used for `input_audio` content type.
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    /// ID of a previous conversation item to reference (for `item_reference` content types in `response.create` events). These can reference both client and server created items.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The text content, used for `input_text` and `text` content types.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// The transcript of the audio, used for `input_audio` content type.
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
    /// The content type (`input_text`, `input_audio`, `item_reference`, `text`).
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
