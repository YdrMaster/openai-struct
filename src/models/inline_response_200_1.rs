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
pub struct InlineResponse2001 {
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[serde(rename = "run_id")]
    pub run_id: Option<String>,
}
