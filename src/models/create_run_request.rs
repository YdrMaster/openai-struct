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
pub struct CreateRunRequest {
    /// Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions.
    #[serde(rename = "additional_instructions")]
    pub additional_instructions: Option<String>,
    /// Adds additional messages to the thread before creating the run.
    #[serde(rename = "additional_messages")]
    pub additional_messages: Option<Vec<crate::models::CreateMessageRequest>>,
    /// The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// Overrides the [instructions](/docs/api-reference/assistants/createAssistant) of the assistant. This is useful for modifying the behavior on a per-run basis.
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    /// The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<i32>,
    /// The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.
    #[serde(rename = "max_prompt_tokens")]
    pub max_prompt_tokens: Option<i32>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used.
    #[serde(rename = "model")]
    pub model: Option<Value>,
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<crate::models::ParallelToolCalls>,
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<crate::models::ReasoningEffort>,
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::models::AssistantsApiResponseFormatOption>,
    /// If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<crate::models::CreateRunRequestToolChoice>,
    /// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<Value>>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: Option<crate::models::CreateRunRequestToolChoice>,
}
