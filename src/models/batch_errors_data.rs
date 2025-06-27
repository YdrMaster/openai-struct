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
pub struct BatchErrorsData {
    /// An error code identifying the error type.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// The line number of the input file where the error occurred, if applicable.
    #[serde(rename = "line")]
    pub line: Option<i32>,
    /// A human-readable message providing more details about the error.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the parameter that caused the error, if applicable.
    #[serde(rename = "param")]
    pub param: Option<String>,
}
