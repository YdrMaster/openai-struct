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
pub struct FineTuningJobHyperparameters {
    /// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
    #[serde(rename = "batch_size")]
    pub batch_size: Option<crate::StringOrNumber>,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier: Option<crate::StringOrNumber>,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<crate::StringOrNumber>,
}
