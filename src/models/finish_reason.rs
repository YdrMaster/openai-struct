/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.

/// # on openai.yaml
///
/// ```yaml
/// finish_reason:
///   type: string
///   description: >
///     The reason the model stopped generating tokens. This will be
///     `stop` if the model hit a natural stop point or a provided
///     stop sequence,
///
///     `length` if the maximum number of tokens specified in the
///     request was reached,
///
///     `content_filter` if content was omitted due to a flag from our
///     content filters,
///
///     `tool_calls` if the model called a tool, or `function_call`
///     (deprecated) if the model called a function.
///   enum:
///     - stop
///     - length
///     - tool_calls
///     - content_filter
///     - function_call
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    FunctionCall,
}

#[test]
fn test_finish_reason() {
    assert_eq!(
        serde_json::to_string(&FinishReason::ContentFilter)
            .unwrap()
            .as_str(),
        "\"content_filter\""
    );
}
