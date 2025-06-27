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
pub struct CreateEvalRequest {
    /// The configuration for the data source used for the evaluation runs.
    #[serde(rename = "data_source_config")]
    pub data_source_config: Value,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    /// The name of the evaluation.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A list of graders for all eval runs in this group.
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: Vec<Value>,
}
