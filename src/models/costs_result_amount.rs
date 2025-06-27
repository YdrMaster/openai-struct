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
pub struct CostsResultAmount {
    /// Lowercase ISO-4217 currency e.g. \"usd\"
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// The numeric value of the cost.
    #[serde(rename = "value")]
    pub value: Option<f32>,
}
