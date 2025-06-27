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
pub struct ListFilesResponse {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::OpenAiFile>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "object")]
    pub object: String,
}
