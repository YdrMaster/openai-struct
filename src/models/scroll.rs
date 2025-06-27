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
pub struct Scroll {
    /// The horizontal scroll distance.
    #[serde(rename = "scroll_x")]
    pub scroll_x: i32,
    /// The vertical scroll distance.
    #[serde(rename = "scroll_y")]
    pub scroll_y: i32,
    /// Specifies the event type. For a scroll action, this property is  always set to `scroll`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The x-coordinate where the scroll occurred.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate where the scroll occurred.
    #[serde(rename = "y")]
    pub y: i32,
}
