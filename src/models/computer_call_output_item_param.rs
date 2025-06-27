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
/// ComputerCallOutputItemParam:
///   properties:
///     id:
///       anyOf:
///         - type: string
///           description: The ID of the computer tool call output.
///         - type: "null"
///     call_id:
///       type: string
///       maxLength: 64
///       minLength: 1
///       description: The ID of the computer tool call that produced the output.
///     type:
///       type: string
///       enum:
///         - computer_call_output
///       description: The type of the computer tool call output. Always
///         `computer_call_output`.
///       default: computer_call_output
///       x-stainless-const: true
///     output:
///       $ref: "#/components/schemas/ComputerScreenshotImage"
///     acknowledged_safety_checks:
///       anyOf:
///         - items:
///             $ref: "#/components/schemas/ComputerCallSafetyCheckParam"
///           type: array
///           description:
///             The safety checks reported by the API that have been acknowledged
///             by the developer.
///         - type: "null"
///     status:
///       anyOf:
///         - type: string
///           enum:
///             - in_progress
///             - completed
///             - incomplete
///           description:
///             The status of the message input. One of `in_progress`, `completed`,
///             or `incomplete`. Populated when input items are returned via
///             API.
///         - type: "null"
///   type: object
///   required:
///     - call_id
///     - type
///     - output
///   title: Computer tool call output
///   description: The output of a computer tool call.
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerCallOutputItemParam {
    #[serde(rename = "acknowledged_safety_checks")]
    pub acknowledged_safety_checks: Option<Vec<crate::ComputerCallSafetyCheckParam>>,
    /// The ID of the computer tool call that produced the output.
    #[serde(rename = "call_id")]
    pub call_id: String,
    /// type: "null" or type: string of The ID of the computer tool call output.
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "output")]
    pub output: crate::models::ComputerScreenshotImage,
    /// anyOf: string`in_progress`、string`completed`、string`incomplete` or Option`null`
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The type of the computer tool call output. Always `computer_call_output`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "computer_call_output".into()
}
