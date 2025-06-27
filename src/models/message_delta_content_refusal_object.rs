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
pub struct MessageDeltaContentRefusalObject {
    /// The index of the refusal part in the message.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
    /// Always `refusal`.
    #[serde(rename = "type")]
    pub _type: String,
}
