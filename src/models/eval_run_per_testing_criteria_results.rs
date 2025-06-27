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
pub struct EvalRunPerTestingCriteriaResults {
    /// Number of tests failed for this criteria.
    #[serde(rename = "failed")]
    pub failed: i32,
    /// Number of tests passed for this criteria.
    #[serde(rename = "passed")]
    pub passed: i32,
    /// A description of the testing criteria.
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: String,
}
