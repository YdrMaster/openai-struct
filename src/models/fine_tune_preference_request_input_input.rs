/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTunePreferenceRequestInputInput {
    #[serde(rename = "messages")]
    pub messages: Option<Vec<Value>>,
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<crate::models::ParallelToolCalls>,
    /// A list of tools the model may generate JSON inputs for.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<crate::models::ChatCompletionTool>>,
}
