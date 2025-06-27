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
/// AssistantToolsFileSearch:
///   type: object
///   title: FileSearch tool
///   properties:
///     type:
///       type: string
///       description: "The type of tool being defined: `file_search`"
///       enum:
///         - file_search
///       x-stainless-const: true
///     file_search:
///       type: object
///       description: Overrides for the file search tool.
///       properties:
///         max_num_results:
///           type: integer
///           minimum: 1
///           maximum: 50
///           description: |
///             The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.
///
///             Note that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.
///         ranking_options:
///           $ref: "#/components/schemas/FileSearchRankingOptions"
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantToolsFileSearch {
    #[serde(rename = "file_search")]
    pub file_search: Option<crate::models::AssistantToolsFileSearchFileSearch>,
    /// The type of tool being defined: `file_search`
    #[serde(rename = "type")]
    pub _type: String,
}
