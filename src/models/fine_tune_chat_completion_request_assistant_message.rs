/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTuneChatCompletionRequestAssistantMessage {
    #[serde(rename = "audio")]
    pub audio: Option<crate::models::ChatCompletionRequestAssistantMessageAudio>,
    /// The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.
    #[serde(rename = "content")]
    pub content: Option<Value>,
    #[serde(rename = "function_call")]
    pub function_call: Option<crate::models::ChatCompletionRequestAssistantMessageFunctionCall>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The refusal message by the assistant.
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
    /// The role of the messages author, in this case `assistant`.
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<crate::models::ChatCompletionMessageToolCalls>,
}
