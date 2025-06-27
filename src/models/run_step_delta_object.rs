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
pub struct RunStepDeltaObject {
    #[serde(rename = "delta")]
    pub delta: crate::models::RunStepDeltaObjectDelta,
    /// The identifier of the run step, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `thread.run.step.delta`.
    #[serde(rename = "object")]
    pub object: String,
}
