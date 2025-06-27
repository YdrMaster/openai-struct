/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub FunctionTool : Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling).
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionTool {
    #[serde(rename = "parameters")]
    pub parameters: Value,
    #[serde(rename = "description")]
    pub description: Option<Value>,
    /// The name of the function to call.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "strict")]
    pub strict: Value,
    /// The type of the function tool. Always `function`.
    #[serde(rename = "type")]
    pub _type: String,
}
