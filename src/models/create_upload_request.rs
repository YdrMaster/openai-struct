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
pub struct CreateUploadRequest {
    /// The number of bytes in the file you are uploading.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The name of the file to upload.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The MIME type of the file.  This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    /// The intended purpose of the uploaded file.  See the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose).
    #[serde(rename = "purpose")]
    pub purpose: String,
}
