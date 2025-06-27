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
pub struct EvalJsonlFileIdSource {
    /// The identifier of the file.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of jsonl source. Always `file_id`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "file_id".into()
}
