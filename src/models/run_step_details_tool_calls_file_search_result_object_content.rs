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
pub struct RunStepDetailsToolCallsFileSearchResultObjectContent {
    /// The text content of the file.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// The type of the content.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
