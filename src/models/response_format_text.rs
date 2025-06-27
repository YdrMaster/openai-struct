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
/// ResponseFormatText:
///   type: object
///   title: Text
///   description: |
///     Default response format. Used to generate text responses.
///   properties:
///     type:
///       type: string
///       description: The type of response format being defined. Always `text`.
///       enum:
///         - text
///       x-stainless-const: true
///   required:
///     - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormatText {
    Text,
}

#[test]
fn test_test() {
    assert_eq!(
        serde_json::to_string(&ResponseFormatText::Text).unwrap(),
        "\"text\""
    );
}
