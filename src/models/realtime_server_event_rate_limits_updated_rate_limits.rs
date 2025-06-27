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
pub struct RealtimeServerEventRateLimitsUpdatedRateLimits {
    /// The maximum allowed value for the rate limit.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The name of the rate limit (`requests`, `tokens`).
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The remaining value before the limit is reached.
    #[serde(rename = "remaining")]
    pub remaining: Option<i32>,
    /// Seconds until the rate limit resets.
    #[serde(rename = "reset_seconds")]
    pub reset_seconds: Option<f32>,
}
