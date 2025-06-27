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
pub struct EvalLabelModelGrader {
    #[serde(rename = "input")]
    pub input: Vec<crate::models::EvalItem>,
    /// The labels to assign to each item in the evaluation.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// The model to use for the evaluation. Must support structured outputs.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the grader.
    #[serde(rename = "name")]
    pub name: String,
    /// The labels that indicate a passing result. Must be a subset of labels.
    #[serde(rename = "passing_labels")]
    pub passing_labels: Vec<String>,
    /// The object type, which is always `label_model`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "label_model".into()
}
