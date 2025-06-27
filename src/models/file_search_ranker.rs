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
/// FileSearchRanker:
///   type: string
///   description:
///     The ranker to use for the file search. If not specified will use
///     the `auto` ranker.
///   enum:
///     - auto
///     - default_2024_08_21
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileSearchRanker {
    Auto,
    #[serde(rename = "default_2024_08_21")]
    Default20240821,
}

#[test]
fn test_ranker() {
    assert_eq!(
        serde_json::to_string(&FileSearchRanker::Auto).unwrap(),
        "\"auto\""
    );
    assert_eq!(
        serde_json::to_string(&FileSearchRanker::Default20240821).unwrap(),
        "\"default_2024_08_21\""
    );
}
