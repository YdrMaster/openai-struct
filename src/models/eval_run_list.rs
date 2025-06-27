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
pub struct EvalRunList {
    /// An array of eval run objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::EvalRun>,
    /// The identifier of the first eval run in the data array.
    #[serde(rename = "first_id")]
    pub first_id: String,
    /// Indicates whether there are more evals available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// The identifier of the last eval run in the data array.
    #[serde(rename = "last_id")]
    pub last_id: String,
    /// The type of this object. It is always set to \"list\".
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
}

fn default_object() -> String {
    "list".into()
}
