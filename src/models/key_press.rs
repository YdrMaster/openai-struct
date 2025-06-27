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
pub struct KeyPress {
    /// The combination of keys the model is requesting to be pressed. This is an array of strings, each representing a key.
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
    /// Specifies the event type. For a keypress action, this property is  always set to `keypress`.
    #[serde(rename = "type")]
    pub _type: String,
}
