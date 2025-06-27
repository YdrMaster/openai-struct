/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub RunStepDeltaObjectDelta : The delta containing the fields that have changed on the run step.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepDeltaObjectDelta {
    /// The details of the run step.
    #[serde(rename = "step_details")]
    pub step_details: Option<Value>,
}
