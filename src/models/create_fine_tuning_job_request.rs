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
pub struct CreateFineTuningJobRequest {
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: Option<crate::models::CreateFineTuningJobRequestHyperparameters>,
    /// A list of integrations to enable for your fine-tuning job.
    #[serde(rename = "integrations")]
    pub integrations: Option<Vec<crate::models::CreateFineTuningJobRequestIntegrations>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    #[serde(rename = "method")]
    pub method: Option<crate::models::FineTuneMethod>,
    /// The name of the model to fine-tune. You can select one of the [supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned).
    /// enum: - babbage-002 - davinci-002 - gpt-3.5-turbo - gpt-4o-mini
    #[serde(rename = "model")]
    pub model: String,
    /// The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases. If a seed is not specified, one will be generated for you.
    #[serde(rename = "seed")]
    pub seed: Option<i32>,
    /// A string of up to 64 characters that will be added to your fine-tuned model name.  For example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.
    #[serde(rename = "suffix")]
    pub suffix: Option<String>,
    /// The ID of an uploaded file that contains training data.  See [upload file](/docs/api-reference/files/create) for how to upload a file.  Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.  The contents of the file should differ depending on if the model uses the [chat](/docs/api-reference/fine-tuning/chat-input), [completions](/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](/docs/api-reference/fine-tuning/preference-input) format.  See the [fine-tuning guide](/docs/guides/fine-tuning) for more details.
    #[serde(rename = "training_file")]
    pub training_file: String,
    /// The ID of an uploaded file that contains validation data.  If you provide this file, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in the fine-tuning results file. The same data should not be present in both train and validation files.  Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.  See the [fine-tuning guide](/docs/guides/fine-tuning) for more details.
    #[serde(rename = "validation_file")]
    pub validation_file: Option<String>,
}
