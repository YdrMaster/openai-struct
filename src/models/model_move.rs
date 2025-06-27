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
pub struct ModelMove {
    /// Specifies the event type. For a move action, this property is  always set to `move`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The x-coordinate to move to.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate to move to.
    #[serde(rename = "y")]
    pub y: i32,
}
