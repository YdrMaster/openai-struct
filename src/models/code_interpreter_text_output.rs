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
/// CodeInterpreterTextOutput:
///   type: object
///   title: Code interpreter text output
///   description: |
///     The output of a code interpreter tool call that is text.
///   properties:
///     type:
///       type: string
///       enum:
///         - logs
///       description: |
///         The type of the code interpreter text output. Always `logs`.
///       x-stainless-const: true
///     logs:
///       type: string
///       description: |
///         The logs of the code interpreter tool call.
///   required:
///     - type
///     - logs
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInterpreterTextOutput {
    /// The logs of the code interpreter tool call.
    #[serde(rename = "logs")]
    pub logs: String,
    /// The type of the code interpreter text output. Always `logs`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

impl Default for CodeInterpreterTextOutput {
    fn default() -> CodeInterpreterTextOutput {
        Self {
            logs: "".to_string(),
            _type: default_type(),
        }
    }
}

fn default_type() -> String {
    "logs".into()
}
