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
pub struct ChatCompletionResponseMessageUrlCitation {
    /// The index of the last character of the URL citation in the message.
    #[serde(rename = "end_index")]
    pub end_index: i32,
    /// The index of the first character of the URL citation in the message.
    #[serde(rename = "start_index")]
    pub start_index: i32,
    /// The title of the web resource.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL of the web resource.
    #[serde(rename = "url")]
    pub url: String,
}
