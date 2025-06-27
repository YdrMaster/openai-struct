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
pub struct BatchErrors {
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::models::BatchErrorsData>>,
    /// The object type, which is always `list`.
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: Option<String>,
}

fn default_object() -> Option<String> {
    Some("auto".into())
}
