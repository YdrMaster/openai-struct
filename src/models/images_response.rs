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
pub struct ImagesResponse {
    /// The Unix timestamp (in seconds) of when the image was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// The list of generated images.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::models::Image>>,
    #[serde(rename = "usage")]
    pub usage: Option<crate::models::ImagesResponseUsage>,
}
