/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageTimeBucket {
    #[serde(rename = "end_time")]
    pub end_time: i32,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "result")]
    pub result: Vec<Value>,
    #[serde(rename = "start_time")]
    pub start_time: i32,
}
