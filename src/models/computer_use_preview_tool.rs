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
/// ComputerUsePreviewTool:
///   properties:
///     type:
///       type: string
///       enum:
///         - computer_use_preview
///       description: The type of the computer use tool. Always `computer_use_preview`.
///       default: computer_use_preview
///       x-stainless-const: true
///     environment:
///       type: string
///       enum:
///         - windows
///         - mac
///         - linux
///         - ubuntu
///         - browser
///       description: The type of computer environment to control.
///     display_width:
///       type: integer
///       description: The width of the computer display.
///     display_height:
///       type: integer
///       description: The height of the computer display.
///   type: object
///   required:
///     - type
///     - environment
///     - display_width
///     - display_height
///   title: Computer use preview
///   description: A tool that controls a virtual computer. Learn more about the
///     [computer tool](https:///platform.openai.com/docs/guides/tools-computer-use).
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerUsePreviewTool {
    /// The height of the computer display.
    #[serde(rename = "display_height")]
    pub display_height: i32,
    /// The width of the computer display.
    #[serde(rename = "display_width")]
    pub display_width: i32,
    /// The type of computer environment to control.
    #[serde(rename = "environment")]
    pub environment: String,
    /// The type of the computer use tool. Always `computer_use_preview`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "computer_use_preview".into()
}
