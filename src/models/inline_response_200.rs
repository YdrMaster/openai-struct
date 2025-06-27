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
pub struct InlineResponse200 {
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "eval_id")]
    pub eval_id: String,
    #[serde(rename = "object")]
    pub object: String,
}
