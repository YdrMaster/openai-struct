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
pub struct AssistantsNamedToolChoice {
    #[serde(rename = "function")]
    pub function: Option<crate::models::AssistantsNamedToolChoiceFunction>,
    /// The type of the tool. If type is `function`, the function name must be set
    #[serde(rename = "type")]
    pub _type: String,
}
