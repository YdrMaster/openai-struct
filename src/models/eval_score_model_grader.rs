/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub EvalScoreModelGrader : A ScoreModelGrader object that uses a model to assign a score to the input.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalScoreModelGrader {
    /// The input text. This may include template strings.
    #[serde(rename = "input")]
    pub input: Vec<crate::models::EvalItem>,
    /// The model to use for the evaluation.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the grader.
    #[serde(rename = "name")]
    pub name: String,
    /// The threshold for the score.
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: Option<f32>,
    /// The range of the score. Defaults to `[0, 1]`.
    #[serde(rename = "range")]
    #[serde(default = "default_range")]
    pub range: Option<Vec<f32>>,
    /// The sampling parameters for the model.
    #[serde(rename = "sampling_params")]
    pub sampling_params: Option<Value>,
    /// The object type, which is always `score_model`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "score_model".into()
}

fn default_range() -> Option<Vec<f32>> {
    Some(vec![0f32, 1f32])
}
