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
pub struct CreateFineTuningJobRequestIntegrations {
    /// The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.
    #[serde(rename = "type")]
    pub _type: Value,
    #[serde(rename = "wandb")]
    pub wandb: crate::models::CreateFineTuningJobRequestWandb,
}
