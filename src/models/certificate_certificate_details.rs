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
pub struct CertificateCertificateDetails {
    /// The content of the certificate in PEM format.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// The Unix timestamp (in seconds) of when the certificate expires.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i32>,
    /// The Unix timestamp (in seconds) of when the certificate becomes valid.
    #[serde(rename = "valid_at")]
    pub valid_at: Option<i32>,
}
