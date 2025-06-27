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
/// ComputerScreenshotImage:
///   type: object
///   description: |
///     A computer screenshot image used with the computer use tool.
///   properties:
///     type:
///       type: string
///       enum:
///         - computer_screenshot
///       default: computer_screenshot
///       description: >
///         Specifies the event type. For a computer screenshot, this property
///         is
///
///         always set to `computer_screenshot`.
///       x-stainless-const: true
///     image_url:
///       type: string
///       description: The URL of the screenshot image.
///     file_id:
///       type: string
///       description: The identifier of an uploaded file that contains the screenshot.
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerScreenshotImage {
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Specifies the event type. For a computer screenshot, this property is  always set to `computer_screenshot`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "computer_screenshot".into()
}
