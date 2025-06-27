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
/// ComputerToolCall:
///   type: object
///   title: Computer tool call
///   description: >
///     A tool call to a computer use tool. See the
///
///     [computer use guide](/docs/guides/tools-computer-use) for more
///     information.
///   properties:
///     type:
///       type: string
///       description: The type of the computer call. Always `computer_call`.
///       enum:
///         - computer_call
///       default: computer_call
///     id:
///       type: string
///       description: The unique ID of the computer call.
///     call_id:
///       type: string
///       description: |
///         An identifier used when responding to the tool call with output.
///     action:
///       $ref: "#/components/schemas/ComputerAction"
///     pending_safety_checks:
///       type: array
///       items:
///         $ref: "#/components/schemas/ComputerToolCallSafetyCheck"
///       description: |
///         The pending safety checks for the computer call.
///     status:
///       type: string
///       description: |
///         The status of the item. One of `in_progress`, `completed`, or
///         `incomplete`. Populated when items are returned via API.
///       enum:
///         - in_progress
///         - completed
///         - incomplete
///   required:
///     - type
///     - id
///     - action
///     - call_id
///     - pending_safety_checks
///     - status
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolCall {
    #[serde(rename = "action")]
    pub action: crate::models::ComputerAction,
    /// An identifier used when responding to the tool call with output.
    #[serde(rename = "call_id")]
    pub call_id: String,
    /// The unique ID of the computer call.
    #[serde(rename = "id")]
    pub id: String,
    /// The pending safety checks for the computer call.
    #[serde(rename = "pending_safety_checks")]
    pub pending_safety_checks: Vec<crate::models::ComputerToolCallSafetyCheck>,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(rename = "status")]
    pub status: String,
    /// The type of the computer call. Always `computer_call`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "computer_call".into()
}
