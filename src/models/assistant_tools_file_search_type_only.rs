/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// AssistantToolsFileSearchTypeOnly:
///   type: object
///   title: FileSearch tool
///   properties:
///     type:
///       type: string
///       description: "The type of tool being defined: `file_search`"
///       enum:
///         - file_search
///       x-stainless-const: true
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantToolsFileSearchTypeOnly {
    /// The type of tool being defined: `file_search`
    #[serde(rename = "type")]
    pub _type: String,
}
