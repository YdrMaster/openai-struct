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
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    /// The ID of the message that was created by this run step.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
}
