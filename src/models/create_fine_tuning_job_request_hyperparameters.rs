/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub CreateFineTuningJobRequestHyperparameters : The hyperparameters used for the fine-tuning job. This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequestHyperparameters {
    /// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
    #[serde(rename = "batch_size")]
    pub batch_size: Option<Value>,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier: Option<Value>,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<Value>,
}
