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
pub struct DeleteCertificateResponse {
    /// The ID of the certificate that was deleted.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, must be `certificate.deleted`.
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
}

fn default_object() -> String {
    "certificate.deleted".into()
}
