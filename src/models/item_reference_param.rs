/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// pub ItemReferenceParam : An internal identifier for an item to reference.
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemReferenceParam {
    /// The ID of the item to reference.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Option<Value>,
}
