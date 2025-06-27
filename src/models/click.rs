/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
///     Click:
///       type: object
///       title: Click
///       description: |
///         A click action.
///       properties:
///         type:
///           type: string
///           enum:
///             - click
///           default: click
///           description: |
///             Specifies the event type. For a click action, this property is
///             always set to `click`.
///           x-stainless-const: true
///         button:
///           type: string
///           enum:
///             - left
///             - right
///             - wheel
///             - back
///             - forward
///           description: >
///             Indicates which mouse button was pressed during the click. One of
///             `left`, `right`, `wheel`, `back`, or `forward`.
///         x:
///           type: integer
///           description: |
///             The x-coordinate where the click occurred.
///         y:
///           type: integer
///           description: |
///             The y-coordinate where the click occurred.
///       required:
///         - type
///         - button
///         - x
///         - y
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Click {
    /// Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.
    #[serde(rename = "button")]
    pub button: String,
    /// Specifies the event type. For a click action, this property is  always set to `click`.
    #[serde(rename = "type")]
    pub _type: String,
    /// The x-coordinate where the click occurred.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate where the click occurred.
    #[serde(rename = "y")]
    pub y: i32,
}
