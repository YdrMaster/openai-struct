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
pub struct RealtimeResponseCreateParamsTools {
    /// Parameters of the function in JSON Schema.
    #[serde(rename = "parameters")]
    pub parameters: Option<Value>,
    /// The description of the function, including guidance on when and how  to call it, and guidance about what to tell the user when calling  (if anything).
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of the tool, i.e. `function`.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
