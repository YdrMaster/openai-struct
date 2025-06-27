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
/// ChatCompletionRequestToolMessage:
///   type: object
///   title: Tool message
///   properties:
///     role:
///       type: string
///       enum:
///         - tool
///       description: The role of the messages author, in this case `tool`.
///       x-stainless-const: true
///     content:
///       oneOf:
///         - type: string
///           description: The contents of the tool message.
///           title: Text content
///         - type: array
///           description:
///             An array of content parts with a defined type. For tool messages,
///             only type `text` is supported.
///           title: Array of content parts
///           items:
///             $ref: "#/components/schemas/ChatCompletionRequestToolMessageContentPart"
///           minItems: 1
///       description: The contents of the tool message.
///     tool_call_id:
///       type: string
///       description: Tool call that this message is responding to.
///   required:
///     - role
///     - content
///     - tool_call_id
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestToolMessage {
    /// The contents of the tool message.
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestToolMessageContent,
    /// Tool call that this message is responding to.
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: String,
}

/// # on openapi.yaml
///
/// ```yaml
/// content:
///   oneOf:
///     - type: string
///       description: The contents of the tool message.
///       title: Text content
///     - type: array
///       description:
///         An array of content parts with a defined type. For tool messages,
///         only type `text` is supported.
///       title: Array of content parts
///       items:
///         $ref: "#/components/schemas/ChatCompletionRequestToolMessageContentPart"
///       minItems: 1
///   description: The contents of the tool message.
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    Text(String),
    Array(Vec<crate::ChatCompletionRequestToolMessageContentPart>),
}

#[test]
fn chat_message_developer_message_content() {
    assert_eq!(
        serde_json::to_string(&ChatCompletionRequestToolMessage {
            content: ChatCompletionRequestToolMessageContent::Array(vec![
                crate::ChatCompletionRequestToolMessageContentPart::Text(
                    crate::ChatCompletionRequestMessageContentPartText {
                        text: "233".into(),
                        _type: "qw".into(),
                    }
                ),
                crate::ChatCompletionRequestToolMessageContentPart::Text(
                    crate::ChatCompletionRequestMessageContentPartText {
                        text: "emm".into(),
                        _type: "hk".into(),
                    }
                ),
            ]),
            tool_call_id: "1228".into(),
        })
        .unwrap(),
        r#"{"content":[{"text":"233","type":"qw"},{"text":"emm","type":"hk"}],"tool_call_id":"1228"}"#
    );
}
