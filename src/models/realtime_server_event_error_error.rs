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
pub struct RealtimeServerEventErrorError {
    /// Error code, if any.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// The event_id of the client event that caused the error, if applicable.
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    /// A human-readable error message.
    #[serde(rename = "message")]
    pub message: String,
    /// Parameter related to the error, if any.
    #[serde(rename = "param")]
    pub param: Option<String>,
    /// The type of error (e.g., \"invalid_request_error\", \"server_error\").
    #[serde(rename = "type")]
    pub _type: String,
}
