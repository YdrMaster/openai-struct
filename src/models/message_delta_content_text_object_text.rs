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
pub struct MessageDeltaContentTextObjectText {
    #[serde(rename = "annotations")]
    pub annotations: Option<Vec<Value>>,
    /// The data that makes up the text.
    #[serde(rename = "value")]
    pub value: Option<String>,
}
