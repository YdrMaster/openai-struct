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
pub struct FileSearchToolCallResults {
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::models::VectorStoreFileAttributes>,
    /// The unique ID of the file.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The name of the file.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// The relevance score of the file - a value between 0 and 1.
    #[serde(rename = "score")]
    pub score: Option<f32>,
    /// The text that was retrieved from the file.
    #[serde(rename = "text")]
    pub text: Option<String>,
}
