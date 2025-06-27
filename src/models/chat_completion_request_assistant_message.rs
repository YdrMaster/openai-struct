/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestAssistantMessage {
    #[serde(rename = "audio")]
    pub audio: Option<crate::models::ChatCompletionRequestAssistantMessageAudio>,
    /// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
    #[serde(rename = "content")]
    pub content: Option<ChatCompletionRequestAssistantMessageContent>,
    #[serde(rename = "function_call")]
    pub function_call: Option<crate::models::ChatCompletionRequestAssistantMessageFunctionCall>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The refusal message by the assistant.
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<crate::models::ChatCompletionMessageToolCalls>,
}

impl Default for ChatCompletionRequestAssistantMessage {
    fn default() -> Self {
        Self {
            audio: None,
            content: None,
            function_call: None,
            name: None,
            refusal: None,
            tool_calls: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    /// The contents of the assistant message.
    Text(String),
    /// An array of content parts with a defined type. Can be one or more of type `text`, or exactly one of type `refusal`.
    Array(Vec<crate::ChatCompletionRequestAssistantMessageContentPart>),
}

impl Default for ChatCompletionRequestAssistantMessageContent {
    fn default() -> Self {
        Self::Text(String::default())
    }
}
