/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalResponsesSource : A EvalResponsesSource object describing a run data source configuration.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalResponsesSource {
    /// Whether to allow parallel tool calls. This is a query parameter used to select responses.
    #[serde(rename = "allow_parallel_tool_calls")]
    pub allow_parallel_tool_calls: Option<bool>,
    /// Only include items created after this timestamp (inclusive). This is a query parameter used to select responses.
    #[serde(rename = "created_after")]
    pub created_after: Option<i32>,
    /// Only include items created before this timestamp (inclusive). This is a query parameter used to select responses.
    #[serde(rename = "created_before")]
    pub created_before: Option<i32>,
    /// Whether the response has tool calls. This is a query parameter used to select responses.
    #[serde(rename = "has_tool_calls")]
    pub has_tool_calls: Option<bool>,
    /// Optional search string for instructions. This is a query parameter used to select responses.
    #[serde(rename = "instructions_search")]
    pub instructions_search: Option<String>,
    /// Metadata filter for the responses. This is a query parameter used to select responses.
    #[serde(rename = "metadata")]
    pub metadata: Option<Value>,
    /// The name of the model to find responses for. This is a query parameter used to select responses.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// Optional reasoning effort parameter. This is a query parameter used to select responses.
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<crate::models::ReasoningEffort>,
    /// Sampling temperature. This is a query parameter used to select responses.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter. This is a query parameter used to select responses.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
    /// The type of run data source. Always `responses`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
    /// List of user identifiers. This is a query parameter used to select responses.
    #[serde(rename = "users")]
    pub users: Option<Vec<String>>,
}

fn default_type() -> String {
    "responses".into()
}
