/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalRun : A schema representing an evaluation run.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalRun {
    /// Unix timestamp (in seconds) when the evaluation run was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// Information about the run's data source.
    #[serde(rename = "data_source")]
    pub data_source: Value,
    #[serde(rename = "error")]
    pub error: crate::models::EvalApiError,
    /// The identifier of the associated evaluation.
    #[serde(rename = "eval_id")]
    pub eval_id: String,
    /// Unique identifier for the evaluation run.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The model that is evaluated, if applicable.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the evaluation run.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the object. Always \"eval.run\".
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
    /// Usage statistics for each model during the evaluation run.
    #[serde(rename = "per_model_usage")]
    pub per_model_usage: Vec<crate::models::EvalRunPerModelUsage>,
    /// Results per testing criteria applied during the evaluation run.
    #[serde(rename = "per_testing_criteria_results")]
    pub per_testing_criteria_results: Vec<crate::models::EvalRunPerTestingCriteriaResults>,
    /// The URL to the rendered evaluation run report on the UI dashboard.
    #[serde(rename = "report_url")]
    pub report_url: String,
    #[serde(rename = "result_counts")]
    pub result_counts: crate::models::EvalRunResultCounts,
    /// The status of the evaluation run.
    #[serde(rename = "status")]
    pub status: String,
}

fn default_object() -> String {
    "eval.run".into()
}
