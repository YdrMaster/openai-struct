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
pub struct EvalStringCheckGrader {
    /// The input text. This may include template strings.
    #[serde(rename = "input")]
    pub input: String,
    /// The name of the grader.
    #[serde(rename = "name")]
    pub name: String,
    /// The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`.
    #[serde(rename = "operation")]
    pub operation: String,
    /// The reference text. This may include template strings.
    #[serde(rename = "reference")]
    pub reference: String,
    /// The object type, which is always `string_check`.
    #[serde(default = "default_type")]
    #[serde(rename = "type")]
    pub _type: String,
}

fn default_type() -> String {
    "string_check".into()
}
