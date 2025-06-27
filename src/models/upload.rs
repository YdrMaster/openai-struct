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
pub struct Upload {
    /// The intended number of bytes to be uploaded.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The Unix timestamp (in seconds) for when the Upload was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The Unix timestamp (in seconds) for when the Upload will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: i32,
    #[serde(rename = "file")]
    pub file: Option<crate::models::CreateRunRequestToolChoice>,
    /// The name of the file to be uploaded.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The Upload unique identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always \"upload\".
    #[serde(rename = "object")]
    pub object: Option<String>,
    /// The intended purpose of the file. [Please refer here](/docs/api-reference/files/object#files/object-purpose) for acceptable values.
    #[serde(rename = "purpose")]
    pub purpose: String,
    /// The status of the Upload.
    #[serde(rename = "status")]
    pub status: String,
}
