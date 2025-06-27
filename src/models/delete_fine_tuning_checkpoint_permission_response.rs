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
pub struct DeleteFineTuningCheckpointPermissionResponse {
    /// Whether the fine-tuned model checkpoint permission was successfully deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// The ID of the fine-tuned model checkpoint permission that was deleted.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always \"checkpoint.permission\".
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
}

fn default_object() -> String {
    "checkpoint.permission".into()
}
