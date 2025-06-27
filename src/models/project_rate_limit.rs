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
pub struct ProjectRateLimit {
    /// The maximum batch input tokens per day. Only present for relevant models.
    #[serde(rename = "batch_1_day_max_input_tokens")]
    pub batch_1_day_max_input_tokens: Option<i32>,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The maximum audio megabytes per minute. Only present for relevant models.
    #[serde(rename = "max_audio_megabytes_per_1_minute")]
    pub max_audio_megabytes_per_1_minute: Option<i32>,
    /// The maximum images per minute. Only present for relevant models.
    #[serde(rename = "max_images_per_1_minute")]
    pub max_images_per_1_minute: Option<i32>,
    /// The maximum requests per day. Only present for relevant models.
    #[serde(rename = "max_requests_per_1_day")]
    pub max_requests_per_1_day: Option<i32>,
    /// The maximum requests per minute.
    #[serde(rename = "max_requests_per_1_minute")]
    pub max_requests_per_1_minute: i32,
    /// The maximum tokens per minute.
    #[serde(rename = "max_tokens_per_1_minute")]
    pub max_tokens_per_1_minute: i32,
    /// The model this rate limit applies to.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always `project.rate_limit`
    #[serde(rename = "object")]
    pub object: String,
}
