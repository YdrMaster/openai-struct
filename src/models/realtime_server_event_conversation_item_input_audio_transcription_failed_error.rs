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
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    /// Error code, if any.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// A human-readable error message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Parameter related to the error, if any.
    #[serde(rename = "param")]
    pub param: Option<String>,
    /// The type of error.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
