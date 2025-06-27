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
pub struct ModelType {
    /// The text to type.
    #[serde(rename = "text")]
    pub text: String,
    /// Specifies the event type. For a type action, this property is  always set to `type`.
    #[serde(rename = "type")]
    pub _type: String,
}
