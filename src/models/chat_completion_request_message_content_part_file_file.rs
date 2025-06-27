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
pub struct ChatCompletionRequestMessageContentPartFileFile {
    /// The base64 encoded file data, used when passing the file to the model  as a string.
    #[serde(rename = "file_data")]
    pub file_data: Option<String>,
    /// The ID of an uploaded file to use as input.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The name of the file, used when passing the file to the model as a  string.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
}
