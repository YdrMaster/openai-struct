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
pub struct CreateModerationResponse {
    /// The unique identifier for the moderation request.
    #[serde(rename = "id")]
    pub id: String,
    /// The model used to generate the moderation results.
    #[serde(rename = "model")]
    pub model: String,
    /// A list of moderation objects.
    #[serde(rename = "results")]
    pub results: Vec<crate::models::CreateModerationResponseResults>,
}
