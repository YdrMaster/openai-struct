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
pub struct UploadCertificateRequest {
    /// The certificate content in PEM format
    #[serde(rename = "content")]
    pub content: String,
    /// An optional name for the certificate
    #[serde(rename = "name")]
    pub name: Option<String>,
}
