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
pub struct FineTuneMethod {
    #[serde(rename = "dpo")]
    pub dpo: Option<crate::models::FineTuneDpoMethod>,
    #[serde(rename = "supervised")]
    pub supervised: Option<crate::models::FineTuneSupervisedMethod>,
    /// The type of method. Is either `supervised` or `dpo`.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
