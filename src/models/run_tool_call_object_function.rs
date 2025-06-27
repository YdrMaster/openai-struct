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
pub struct RunToolCallObjectFunction {
    /// The arguments that the model expects you to pass to the function.
    #[serde(rename = "arguments")]
    pub arguments: String,
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: String,
}
