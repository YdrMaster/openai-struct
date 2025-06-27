/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFile {
    #[serde(rename = "file")]
    pub file: crate::models::ChatCompletionRequestMessageContentPartFileFile,
    /// The type of the content part. Always `file`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "file".into()
}
