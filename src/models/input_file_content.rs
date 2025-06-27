/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub InputFileContent : A file input to the model.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFileContent {
    /// The content of the file to be sent to the model.
    #[serde(rename = "file_data")]
    pub file_data: Option<String>,
    #[serde(rename = "file_id")]
    pub file_id: Option<Value>,
    /// The name of the file to be sent to the model.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub _type: String,
}
