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
pub struct InputTextContent {
    /// The text input to the model.
    #[serde(rename = "text")]
    pub text: String,
    /// The type of the input item. Always `input_text`.
    #[serde(rename = "type")]
    pub _type: String,
}
