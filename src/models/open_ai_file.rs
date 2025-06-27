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
pub struct OpenAiFile {
    /// The size of the file, in bytes.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The Unix timestamp (in seconds) for when the file was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The Unix timestamp (in seconds) for when the file will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i32>,
    /// The name of the file.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The file identifier, which can be referenced in the API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `file`.
    #[serde(rename = "object")]
    pub object: String,
    /// The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`.
    #[serde(rename = "purpose")]
    pub purpose: String,
    /// Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`.
    #[serde(rename = "status")]
    pub status: String,
    /// Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`.
    #[serde(rename = "status_details")]
    pub status_details: Option<String>,
}
