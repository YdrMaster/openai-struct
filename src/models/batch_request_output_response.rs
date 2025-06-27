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
pub struct BatchRequestOutputResponse {
    /// The JSON body of the response
    #[serde(rename = "body")]
    pub body: Option<Value>,
    /// An unique identifier for the OpenAI API request. Please include this request ID when contacting support.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The HTTP status code of the response
    #[serde(rename = "status_code")]
    pub status_code: Option<i32>,
}
