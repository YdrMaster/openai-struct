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
pub struct EvalStoredCompletionsSource {
    /// An optional Unix timestamp to filter items created after this time.
    #[serde(rename = "created_after")]
    pub created_after: Option<i32>,
    /// An optional Unix timestamp to filter items created before this time.
    #[serde(rename = "created_before")]
    pub created_before: Option<i32>,
    /// An optional maximum number of items to return.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// An optional model to filter by (e.g., 'gpt-4o').
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// The type of source. Always `stored_completions`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "stored_completions".into()
}
