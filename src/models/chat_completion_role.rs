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
/// ChatCompletionRole:
///   type: string
///   description: The role of the author of a message
///   enum:
///     - developer
///     - system
///     - user
///     - assistant
///     - tool
///     - function
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompletionRole {
    Developer,
    System,
    User,
    Assistant,
    Tool,
    Function,
}

#[test]
fn test_role() {
    assert_eq!(
        serde_json::to_string(&ChatCompletionRole::Developer).unwrap(),
        r#""developer""#
    );
}
