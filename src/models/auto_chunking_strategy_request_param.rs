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
/// AutoChunkingStrategyRequestParam:
///   type: object
///   title: Auto Chunking Strategy
///   description: The default strategy. This strategy currently uses a
///     `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`.
///   additionalProperties: false
///   properties:
///     type:
///       type: string
///       description: Always `auto`.
///       enum:
///         - auto
///       x-stainless-const: true
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoChunkingStrategyRequestParam {
    /// Always `auto`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

impl Default for AutoChunkingStrategyRequestParam {
    fn default() -> Self {
        Self {
            _type: default_type(),
        }
    }
}

fn default_type() -> String {
    "auto".into()
}

#[test]
fn test_param() {
    assert_eq!(
        serde_json::to_string(&AutoChunkingStrategyRequestParam {
            ..Default::default()
        })
        .unwrap(),
        r#"{"type":"auto"}"#
    );
}
