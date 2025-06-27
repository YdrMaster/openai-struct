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
pub struct RunStepDetailsToolCallsFunctionObject {
    #[serde(rename = "function")]
    pub function: crate::models::RunStepDetailsToolCallsFunctionObjectFunction,
    /// The ID of the tool call object.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of tool call. This is always going to be `function` for this type of tool call.
    #[serde(rename = "type")]
    pub _type: String,
}
