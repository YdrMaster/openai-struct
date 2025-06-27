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
pub struct FineTuningJobCheckpointMetrics {
    #[serde(rename = "full_valid_loss")]
    pub full_valid_loss: Option<f32>,
    #[serde(rename = "full_valid_mean_token_accuracy")]
    pub full_valid_mean_token_accuracy: Option<f32>,
    #[serde(rename = "step")]
    pub step: Option<f32>,
    #[serde(rename = "train_loss")]
    pub train_loss: Option<f32>,
    #[serde(rename = "train_mean_token_accuracy")]
    pub train_mean_token_accuracy: Option<f32>,
    #[serde(rename = "valid_loss")]
    pub valid_loss: Option<f32>,
    #[serde(rename = "valid_mean_token_accuracy")]
    pub valid_mean_token_accuracy: Option<f32>,
}
