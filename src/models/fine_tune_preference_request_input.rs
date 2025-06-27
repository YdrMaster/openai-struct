/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct FineTunePreferenceRequestInput {
    #[serde(rename = "input")]
    pub input: Option<crate::models::FineTunePreferenceRequestInputInput>,
    /// The non-preferred completion message for the output.
    #[serde(rename = "non_preferred_completion")]
    pub non_preferred_completion: Option<Vec<crate::ChatCompletionRequestAssistantMessage>>,
    /// The preferred completion message for the output.
    #[serde(rename = "preferred_completion")]
    pub preferred_completion: Option<Vec<crate::ChatCompletionRequestAssistantMessage>>,
}
