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
pub struct DoubleClick {
    /// Specifies the event type. For a double click action, this property is  always set to `double_click`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
    /// The x-coordinate where the double click occurred.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate where the double click occurred.
    #[serde(rename = "y")]
    pub y: i32,
}

fn default_type() -> String {
    "double_click".into()
}
