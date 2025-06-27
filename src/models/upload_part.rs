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
pub struct UploadPart {
    /// The Unix timestamp (in seconds) for when the Part was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The upload Part unique identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `upload.part`.
    #[serde(rename = "object")]
    pub object: String,
    /// The ID of the Upload object that this Part was added to.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
}
