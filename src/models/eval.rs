/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub Eval : An Eval object with a data source config and testing criteria. An Eval represents a task to be done for your LLM integration. Like:  - Improve the quality of my chatbot  - See how well my chatbot handles customer support  - Check if o3-mini is better at my usecase than gpt-4o
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Eval {
    /// The Unix timestamp (in seconds) for when the eval was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// Configuration of data sources used in runs of the evaluation.
    #[serde(rename = "data_source_config")]
    pub data_source_config: Value,
    /// Unique identifier for the evaluation.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The name of the evaluation.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type.
    #[serde(rename = "object")]
    pub object: String,
    /// A list of testing criteria.
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: Vec<Value>,
}
