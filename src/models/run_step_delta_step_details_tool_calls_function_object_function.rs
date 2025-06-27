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
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
    /// The arguments passed to the function.
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet.
    #[serde(rename = "output")]
    pub output: Option<String>,
}
