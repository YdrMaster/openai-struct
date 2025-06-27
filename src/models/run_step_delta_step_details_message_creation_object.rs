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
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    #[serde(rename = "message_creation")]
    pub message_creation:
        Option<crate::models::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>,
    /// Always `message_creation`.
    #[serde(rename = "type")]
    pub _type: String,
}
